use warp::Filter;

#[tokio::main]
async fn main() {
    // Register the various routes in wrap.

    // homepage at /
    let home = warp::path::end().map(|| "Bienvenue au manuel!");

    // A given text at texte/author/id
    let texte = warp::path!("texte" / String / String )
        .map(|_a, _b| "Unimplemented!");

    // Une notion du programme
    let notion = warp::path!("notion" / String ).
        map(|_| "Unimplemented!");

    // Un semestre du programme de HLP
    let semestre = warp::path!("hlp" / String ).
        and(warp::path::end()).
        map(|_| "Unimplemented!");

    // Un chapitre du programme de HLP
    let partie = warp::path!("hlp" / String / String ).
        map(|_, _| "Unimplemented!");

    let routes = warp::get().and(home
                                 .or(texte)
                                 .or(notion)
                                 .or(semestre)
                                 .or(partie));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
