use crate::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use handlebars::Handlebars;
use lettre::SmtpClient;
use lettre_email::Email;
use pulldown_cmark::{html, Options, Parser};
use std::env;
use urlencoding::encode;
use validator::validate_email;
use warp::{hyper::StatusCode, Filter};

#[derive(Debug)]
struct ErrorReply<'a> {
    code: StatusCode,
    message: Option<&'a str>,
}

impl warp::reject::Reject for ErrorReply<'static> {}

impl Default for ErrorReply<'_> {
    fn default() -> Self {
        ErrorReply {
            code: StatusCode::BAD_REQUEST,
            message: None,
        }
    }
}

/// Helper function to fail with a machine-readable message.
pub fn reply_error(message: &'static str) -> warp::reject::Rejection {
    let reply = ErrorReply {
        message: Some(&message),
        ..ErrorReply::default()
    };

    warp::reject::custom(reply)
}

/// Final filters for all routes.  Convert our ErrorReply into
/// machine-parsable JSON, or a 500 error in all other cases.
pub async fn handle_rejections(
    r: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    let cause = r.find::<ErrorReply>();

    let cause = if cause.is_some() {
        cause.unwrap()
    } else {
        &ErrorReply {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: Some(r#"InternalError"#),
        }
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&json!({"success": false, "error": cause.message})),
        cause.code,
    ))
}

/// Register a new account.
pub async fn auth_register(
    conn: PgConnection,
    form: AuthRegisterForm,
) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::users::dsl::*;

    // Check email
    if !validate_email(&form.email) {
        return Err(reply_error("BadEmail"));
    };

    // Reject empty password
    if form.password.chars().count() == 0 {
        return Err(reply_error("EmptyPassword"));
    };

    #[derive(Queryable)]
    struct Result {
        id: i32,
        display_name: String,
        email: String,
        password: String,
    }

    let result = diesel::insert_into(users)
        .values((
            display_name.eq(form.display_name),
            email.eq(form.email),
            password.eq(pw_hash(form.password.as_str())),
            challenge_code.eq(make_random_id().unwrap()),
        ))
        .returning((id, display_name, email, challenge_code))
        .get_result::<Result>(&conn)
        .map_err(|_| reply_error("EmailAlreadyExists"))?;

    // Send verification email.
    let template = include_str!("../server_templates/email-address-confirmation-email.md");

    send_mail(
        (&result.email, &result.display_name),
        r#"Inscription sur Philosopher.fr"#,
        template,
        &json!({"id": result.id, "name": result.display_name, "code": encode(&result.password)}),
    )
        .map_err(|_| reply_error("CannotSendMail"))?;

    Ok(warp::reply::json(&true))
}

/// Activate an account using the code received by e-mail.
pub async fn auth_activate(
    conn: PgConnection,
    form: AuthActivationForm,
) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::users::dsl::*;

    let c = diesel::update(
        users
            .filter(active.eq(false))
            .filter(id.eq(form.id))
            .filter(challenge_code.eq(form.secret)))
        .set(active.eq(true))
        .execute(&conn)
        .unwrap(); //@FIXME

    if c==1 {
        Ok(warp::reply::json(&true))
    } else {
        Ok(warp::reply::json(&false))
    }
}

/// Open a session with a user credentials.
pub async fn auth_login(
    conn: PgConnection,
    form: AuthLoginForm,
) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::sessions::dsl::*;
    use crate::schema::users::dsl::*;

    // Find user
    let user_id = users
        .filter(email.eq(form.email))
        .filter(password.eq(pw_hash(&form.password)))
        .filter(active.eq(true))
        .select(crate::schema::users::dsl::id)
        .first::<i32>(&conn)
        .map_err(|_| reply_error("BadCredentials"))?;

    // Create session
    let session_id = diesel::insert_into(sessions)
        .values((
            secret.eq(make_random_id().map_err(|_| warp::reject())?),
            sess_user.eq(user_id),
        ))
        .returning(secret)
        .get_result::<String>(&conn)
        .map_err(|_| reply_error("ServerError"))?;

    Ok(warp::reply::json(&session_id))
}

/// Create a new author
pub async fn author_create(
    conn: PgConnection,
    form: AuthorForm,
) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::authors::dsl::*;

    let result: i32 = diesel::insert_into(authors)
        .values(form)
        .returning(id)
        .get_result(&conn)
        .unwrap();
    Ok(warp::reply::json(&result))
}

/// (static-ish) Get the official curriculum (notions)
pub async fn prog_notions_get(conn: PgConnection) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::prog_notions::dsl::*;

    let results = prog_notions
        .load::<ProgNotion>(&conn)
        .expect("Error loading posts");

    Ok(warp::reply::json(&results))
}

/// (static-ish) Get the official curriculum (repères)
pub async fn prog_reperes_get(conn: PgConnection) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::models::*;
    use crate::schema::prog_reperes::dsl::*;

    let results = prog_reperes
        .load::<ProgRepere>(&conn)
        .expect("Error loading posts");

    Ok(warp::reply::json(&results))
}

/// (static-ish) Get the official curriculum (HLP)
pub async fn prog_hlp_get(conn: PgConnection) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::prog_hlp::dsl::*;

    let results = prog_hlp
        .load::<ProgHLP>(&conn)
        .expect("Error loading posts");

    Ok(warp::reply::json(&results))
}

pub fn send_mail(
    recipient: (&str, &str),
    subject: &str,
    tpl: &str,
    params: &serde_json::Value,
) -> Result<lettre::smtp::response::Response, lettre::smtp::error::Error> {
    let hb = Handlebars::new();
    let raw_text = hb.render_template(&tpl, &params).unwrap();

    let parser = Parser::new_ext(&raw_text, Options::empty());
    let mut html_body: String = String::new();
    html::push_html(&mut html_body, parser);

    let email = Email::builder()
        .to(recipient)
        .from(("ne-pas-repondre@philosopher.fr", "Philosopher.fr"))
        .subject(subject)
        .alternative(html_body, raw_text)
        .build()
        .unwrap();

    let mut transport =
        lettre::smtp::SmtpTransport::new(SmtpClient::new_simple("smtp.orange.fr").unwrap());

    lettre::Transport::send(&mut transport, email.into())
}

pub fn make_random_id() -> Result<String, getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    Ok(base64::encode(buf))
}

pub fn pw_hash(pw: &str) -> String {
    let config = argon2::Config::default();
    argon2::hash_encoded(pw.as_bytes(), b"RandomSalt", &config).unwrap()
}

// fn pw_verify(cand: &str, hash: &str) -> bool {
//     argon2::verify_encoded(hash, cand.as_bytes()).unwrap()
// }

pub fn connect_db() -> PgConnection {
    // @FIXME Don't crash if PgSql is down.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[tokio::main]
pub async fn server_main() {
    // @TODO Test DB connection before starting up.

    let connect_db_filter = warp::any().map(move || connect_db());

    let prog_notions = warp::path!("programmes" / "notions")
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and_then(&prog_notions_get);

    let prog_reperes = warp::path!("programmes" / "reperes")
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and_then(&prog_reperes_get);

    let prog_hlp = warp::path!("programmes" / "hlp")
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and_then(&prog_hlp_get);

    let put_author = warp::put()
        .and(warp::path("authors"))
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and(warp::body::json())
        .and_then(&author_create);

    let auth_register = warp::post()
        .and(warp::path("auth"))
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and(warp::body::json())
        .and_then(&auth_register);

    let auth_activate = warp::post()
        .and(warp::path("auth"))
        .and(warp::path("activate"))
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and(warp::body::json())
        .and_then(&auth_activate);

    let auth_login = warp::post()
        .and(warp::path("auth"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and(warp::body::json())
        .and_then(&auth_login);

    let get_routes = prog_notions.or(prog_reperes).or(prog_hlp);

    let put_routes = put_author;
    let post_routes = auth_register.or(auth_login).or(auth_activate);

    warp::serve(get_routes.or(put_routes).or(post_routes))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
