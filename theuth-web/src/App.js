import "./App.scss";
import {
  Badge,
  Button,
  Card,
  Col,
  Container,
  Form,
  FormControl,
  Jumbotron,
  ListGroup,
  Nav,
  NavDropdown,
  Navbar,
  Row,
} from "react-bootstrap";
import { QueryClient, QueryClientProvider, useQuery } from "react-query";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { LinkContainer } from "react-router-bootstrap";

const queryClient = new QueryClient();

function Home() {
  const slogans = [
    "Le manuel du&nbsp;peuple, par&nbsp;le&nbsp;peuple, pour&nbsp;le&nbsp;peuple.",
    "Les manuels n'ont fait que décrire le monde, il s'agit maintenant de le changer.",
  ];

  return (
    <div>
      <Jumbotron>
        <h1
          dangerouslySetInnerHTML={{
            __html: slogans[2],
          }}
        ></h1>
        <p class="lead">
          Philosopher.fr est un recueil de textes de philosophie libre, gratuit
          et collaboratif, organisé selon les programme du cycle terminal de
          l'enseignement secondaire français.{" "}
          <LinkContainer to="/meta">
            <a>En savoir plus…</a>
          </LinkContainer>
        </p>
        <p>
          <LinkContainer to="/texte/aleatoire">
            <Button variant="success" size="lg">
              Voir un texte au hasard
            </Button>
          </LinkContainer>
          {" ou "}
          <LinkContainer to="/texte/nouveau">
            <Button class="float-right" variant="success" size="lg">
              Proposer un nouveau texte
            </Button>
          </LinkContainer>
        </p>
      </Jumbotron>

      <Row>
        <Col xs={3}>
          <Card bg="light">
            <Card.Header>
              <strong>Notions</strong>
            </Card.Header>
            <ListGroup variant="flush">
              <ListGroup.Item action>
                L'art <Badge variant="primary">321</Badge>
              </ListGroup.Item>
              <ListGroup.Item action>Le bonheur</ListGroup.Item>
              <ListGroup.Item active action>
                La conscience
              </ListGroup.Item>
            </ListGroup>
          </Card>
          <br />

          <Card bg="light">
            <Card.Header>
              <strong>Repères</strong>
            </Card.Header>
            <ListGroup variant="flush">
              <ListGroup.Item action>Absolu/relatif</ListGroup.Item>
              <ListGroup.Item action>Abstrait/concret</ListGroup.Item>
              <ListGroup.Item active action>
                En acte/en puissance
              </ListGroup.Item>
            </ListGroup>
          </Card>

          <br />

          <Card bg="light">
            <Card.Header>
              <strong>HLP</strong>
            </Card.Header>
            <ListGroup variant="flush">
              <ListGroup.Item action>
                <strong>I. Les pouvoirs de la parole</strong>
              </ListGroup.Item>
              <ListGroup.Item action>L'art de la parole</ListGroup.Item>
              <ListGroup.Item active action>
                La conscience
              </ListGroup.Item>
            </ListGroup>
          </Card>
        </Col>

        <Col>
          {" "}
          <ListGroup>
            <ListGroup.Item action>
              <strong>« Nous ne voyons pas les choses même… »</strong>{" "}
              <Badge pill variant="primary">
                L'art
              </Badge>{" "}
              <Badge pill variant="primary">
                Le langage
              </Badge>
              <br />
              <span class="text-muted">
                Dans ce texte, Bergson indique que toute représentation est par
                nature destructrice de singularité. Il y a quelque chose
                d'unique dans l'expérience
              </span>
            </ListGroup.Item>
            <ListGroup.Item action>Le bonheur</ListGroup.Item>
          </ListGroup>
          14 textes trouvés. Vous pouvez ajouter le vôtre!
        </Col>
      </Row>
    </div>
  );
}

function NumberList(props) {
  const items = props.items;
  const listItems = items.map((item) => (
    <li>
      <a
        href={"/notions/" + item.slug}
        title={"Tous les textes sur " + item.le_name}
      >
        {item.le_name}
      </a>
    </li>
  ));
  return <ul>{listItems}</ul>;
}

function TestPage() {
  const { isLoading, error, data } = useQuery("repoData", () =>
    fetch("http://localhost:3000/categories/philosophie/").then((res) =>
      res.json()
    )
  );
  if (isLoading) return "Loading…";

  if (error) return "An error has occurred: " + error.message;

  return (
    <div className="App">
      <header className="App-header">
        <p>
          <NumberList items={data} />
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

function NavBar() {
  return (
    <Navbar bg="dark" variant="dark" expand="lg">
      <LinkContainer to="/">
        <Navbar.Brand>
          <strong>Philosopher.fr</strong>
        </Navbar.Brand>
      </LinkContainer>
      <Navbar.Toggle aria-controls="basic-navbar-nav" />
      <Navbar.Collapse id="basic-navbar-nav"></Navbar.Collapse>
    </Navbar>
  );
}

function NewText() {
  return (
    <div>
      <Jumbotron>
        <h1>Proposer un texte</h1>
        <p class="lead">
          Cette interface vous permet d’ajouter de nouveaux textes à
          Philosopher.fr. Avant d'ajouter un texte, prenez un moment pour
          vérifier qu'il n'est pas déjà présent dans la base.
        </p>
      </Jumbotron>

      <Form>
        <Form.Group as={Row} controlId="newTextForm.title">
          <Form.Label column sm={2}>
            Un titre pour l'extrait
          </Form.Label>
          <Col sm={10}>
            <Form.Control size="lg" type="text" placeholder="Titre" />
            <Form.Text className="text-muted">
              Le titre sert à présenter ce texte en quelques mots. Il doit être
              bref et précis. <em>Les animaux ne sont que des machines</em> ou{" "}
              <em>Tout ce qu’ordonne le souverain est juste</em> sont de bons
              titres.
            </Form.Text>
          </Col>
        </Form.Group>

        <Form.Group as={Row} controlId="newTextForm.summary">
          <Form.Label column sm={2}>
            {" "}
            Quelques mots pour présenter cet extrait{" "}
          </Form.Label>{" "}
          <Col sm={10}>
            {" "}
            <Form.Control as="textarea" rows={4} placeholder="Présentation" />
            <Form.Text className="text-muted">
              {" "}
              Quelques mots plus détaillés pour présenter cet extrait, son
              intérêt particulier, son contexte ou ce qui fait sa singularité.
              Apparaît dans les résultats de recherche et en « chapô » dans
              l'écran d'affichage.{" "}
            </Form.Text>{" "}
          </Col>{" "}
        </Form.Group>

        <Form.Group as={Row} controlId="newTextForm.body">
          <Form.Label column sm={2}>
            L'extrait lui-même
          </Form.Label>
          <Col sm={10}>
            <Form.Control as="textarea" rows={12} placeholder="Contenu" />
            <Form.Text className="text-muted">
              Mettez uniquement le texte, tel qu’il apparaît dans l’original.
              Pour faire un saut de paragraphe, insérez deux sauts de ligne.
            </Form.Text>
          </Col>
        </Form.Group>

        <h2>Auteur et source</h2>
        <Row>
          <Col>
            <Card bg="light">
              <Card.Header>
                <b>Auteur</b>
              </Card.Header>
              <Card.Body>
                <Form.Group controlId="newTextForm.author">
                  <Form.Label>Auteur</Form.Label>
                  <Form.Control
                    as="select"
                    placeholder="Sélectionnez…"
                    required
                  >
                    <option value="" />
                    <option value="33">Aristote</option>
                    <option>Platon</option>
                    <option>Heidegger</option>
                  </Form.Control>
                  <Form.Text>
                    <a href="#">L’auteur n’est pas dans la liste</a>
                  </Form.Text>
                </Form.Group>

                <Form.Group controlId="newTextForm.pseudo-author">
                  <Form.Label>
                    Certains (rares) textes présentent la doctrine d'un autre
                    auteur que le leur. Si c'est le cas, sélectionnez cet autre
                    auteur ci-dessous: votre textes apparaîtra avec les textes
                    de ce dernier dans la recherche.
                  </Form.Label>
                  <Form.Control as="select">
                    <option>(Ne s'applique pas)</option>{" "}
                  </Form.Control>
                  <Form.Text>Voyez ici pour un exemple.</Form.Text>
                </Form.Group>
              </Card.Body>
            </Card>
          </Col>

          <Col>
            <Card bg="light">
              <Card.Header>
                <b>Origine du texte</b>
              </Card.Header>
              <Card.Body>
                <Form.Group controlId="newTextForm.source">
                  <Form.Label>
                    Livre/article dont est extrait le texte
                  </Form.Label>
                  <Form.Control as="select">
                    <option>Traité du ciel</option>
                  </Form.Control>
                  <Form.Text>
                    <a href="#">L’œuvre n’est pas dans la liste</a>
                  </Form.Text>
                </Form.Group>

                <Form.Group controlId="newTextForm.edition">
                  <Form.Label>
                    Indications sur la traduction ou l'édition
                  </Form.Label>
                  <Form.Control type="text"></Form.Control>
                  <Form.Text className="text-muted">
                    Vous pouvez indiquer ici la traduction citée, ou l'édition
                    particulière dont vous tirez l'extrait.
                  </Form.Text>
                </Form.Group>

                <Form.Group controlId="newTextForm.page">
                  <Form.Label>
                    Numéro de page, de section, de paragraphe…
                  </Form.Label>
                  <Form.Control type="text" />
                  <Form.Text className="text-muted">
                    Vous pouvez indiquer tout ce qui peut être utile à localiser
                    cet extrait précis.  Par exemple: chapitre 2, section XV; Ak
                    XVII, 37; 814b-c…
                  </Form.Text>
                </Form.Group>
              </Card.Body>
            </Card>
          </Col>
        </Row>

        <hr />

        <hr />
      </Form>
    </div>
  );
}

function NotFound() {
  return (
    <div>
      <h1>404</h1>
      <LinkContainer to="/">
        <a>Retour à l’accueil</a>
      </LinkContainer>
    </div>
  );
}

export default function App() {
  return (
    <Router>
      <QueryClientProvider client={queryClient}>
        <NavBar />
        <br />
        <Container>
          {/* A <Switch> looks through its children <Route>s and
              renders the first one that matches the current URL. */}
          <Switch>
            <Route exact path="/texte/nouveau">
              <NewText />
            </Route>
            <Route path="/notions">
              <TestPage />
            </Route>
            <Route exact path="/">
              <Home />
            </Route>
            <Route path="*">
              <NotFound />
            </Route>
          </Switch>
        </Container>
      </QueryClientProvider>
    </Router>
  );
}
