import "./App.scss";
import * as API from "./theuth-api.ts";
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
  Modal,
  Nav,
  NavDropdown,
  Navbar,
  Row,
  Tabs,
  Tab,
} from "react-bootstrap";
import { QueryClient, QueryClientProvider, useQuery } from "react-query";
import { LinkContainer } from "react-router-bootstrap";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { useState } from "react";
import React from "react";

const queryClient = new QueryClient();
const ProgrammeContext = React.createContext({
  notions: null,
  reperes: null,
  hlp: null,
});

function Capitalize(str) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function Hero() {
  const [visible, setVisible] = useState(true);

  const slogans = [
    "Le manuel du&nbsp;peuple, par&nbsp;le&nbsp;peuple, pour&nbsp;le&nbsp;peuple.",
    "Les manuels n'ont fait que décrire le monde, il s'agit maintenant de le changer.",
  ];

  if (!visible) {
    return null;
  }

  return (
    <Jumbotron>
      <button
        type="button"
        class="close"
        aria-label="Close"
        onClick={() => setVisible(false)}
      >
        <span aria-hidden="true">&times;</span>
      </button>
      <h1
        dangerouslySetInnerHTML={{
          __html: slogans[0],
        }}
      ></h1>
      <p class="lead">
        Philosopher.fr est un recueil de textes de philosophie libre, gratuit et
        collaboratif, organisé selon les programme du cycle terminal de
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
          <Button variant="success" size="lg">
            Proposer un nouveau texte
          </Button>
        </LinkContainer>
      </p>
    </Jumbotron>
  );
}

function AppHeader() {
  return (
    <div>
      <Navbar bg="dark" variant="dark" expand="lg">
        <Navbar.Toggle aria-controls="basic-navbar-nav" />
        <LinkContainer to="/">
          <Navbar.Brand>
            <strong>Philosopher.fr</strong>
          </Navbar.Brand>
        </LinkContainer>
        <Navbar.Collapse id="basic-navbar-nav">
          <Nav className="mr-auto">
            {" "}
            <Nav.Link active href="#home">
              Tous les textes
            </Nav.Link>
            <Nav.Link href="#">Proposer un texte</Nav.Link>
          </Nav>
          <Nav className="mr-sm-2"></Nav>
          <Button variant="outline-success" className="mr-sm-2">
            Connexion
          </Button>
          <Button variant="success">Inscription</Button>{" "}
        </Navbar.Collapse>
      </Navbar>

      <br />
      <header>
        {/*
        <h1>
          <strong>Philosopher</strong>.fr
        </h1>
        <hr />

        */}
      </header>
    </div>
  );
}

function Home() {
  return (
    <div>
      <Hero />
      <Row>
        <Col xs={3}>
          <Card bg="light">
            <Card.Header>
              <strong>Filtrer par notions</strong>
            </Card.Header>
            <ListGroup variant="flush">
              <ProgrammeContext.Consumer>
                {(context) =>
                  context.notions.map((notion) => (
                    <ListGroup.Item action>
                      {Capitalize(notion.le_name)}
                    </ListGroup.Item>
                  ))
                }
              </ProgrammeContext.Consumer>
            </ListGroup>
          </Card>
          <br />

          <Card bg="light">
            <Card.Header>
              <strong>Filtrer par repères</strong>
            </Card.Header>
            <ListGroup variant="flush">
              <ProgrammeContext.Consumer>
                {(context) =>
                  context.reperes.map((repere) => (
                    <ListGroup.Item action>
                      {Capitalize(repere.name)}
                    </ListGroup.Item>
                  ))
                }
              </ProgrammeContext.Consumer>
            </ListGroup>
          </Card>
          <br />
          <Card bg="light">
            <Card.Header>
              <strong>Programme HLP</strong>
            </Card.Header>
            <ListGroup variant="flush">
              <ProgrammeContext.Consumer>
                {(context) =>
                  context.hlp.map((partie) => (
                    <ListGroup.Item action>
                      {Capitalize(partie.name)}
                    </ListGroup.Item>
                  ))
                }
              </ProgrammeContext.Consumer>
            </ListGroup>
          </Card>
        </Col>

        <Col>
          {" "}
          <ListGroup>
            <LinkContainer to="/texte/14">
              <ListGroup.Item action>
                <strong>« Nous ne voyons pas les choses même… »</strong>{" "}
                <Badge pill variant="primary">
                  Art
                </Badge>{" "}
                <Badge pill variant="primary">
                  Langage
                </Badge>
                <br />
                <span class="text-muted">
                  Dans ce texte, Bergson indique que toute représentation est
                  par nature destructrice de singularité. Il y a quelque chose
                  d'unique dans l'expérience
                </span>
              </ListGroup.Item>
            </LinkContainer>
            <ListGroup.Item action>Le bonheur</ListGroup.Item>
          </ListGroup>
          14 textes trouvés. Vous pouvez ajouter le vôtre!
        </Col>
      </Row>
    </div>
  );
}

function NewTextEditionForm() {
  return (
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
          Brève présentation de cet extrait (optionnel){" "}
        </Form.Label>{" "}
        <Col sm={10}>
          {" "}
          <Form.Control as="textarea" rows={4} placeholder="Présentation" />
          <Form.Text className="text-muted">
            {" "}
            Un petit paragraphe pour présenter l'intérêt particulier, le
            contexte ou ce qui fait la singularité de cet extrait. Apparaît dans
            les résultats de recherche et en « chapô » dans l'écran d'affichage.
            S'il est vide, le début de l'extrait sera affiché à la place dans la
            recherche.{" "}
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
            Mettez uniquement le texte, tel qu’il apparaît dans l’original. Pour
            faire un saut de paragraphe, insérez deux sauts de ligne.
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
                <Form.Control as="select" placeholder="Sélectionnez…" required>
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
                  auteur ci-dessous: votre textes apparaîtra avec les textes de
                  ce dernier dans la recherche.
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
                <Form.Label>Livre/article dont est extrait le texte</Form.Label>
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
    </Form>
  );
}

function NewAuthor() {
  const [show, setShow] = useState(false);

  const handleClose = () => setShow(false);
  const handleShow = () => setShow(true);

  return (
    <>
      <Button variant="primary" onClick={handleShow}>
        Launch demo modal
      </Button>

      <Modal show={show} onHide={handleClose}>
        <Modal.Header closeButton>
          <Modal.Title>Modal heading</Modal.Title>
        </Modal.Header>
        <Modal.Body>Woohoo, you're reading this text in a modal!</Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={handleClose}>
            Close
          </Button>
          <Button variant="primary" onClick={handleClose}>
            Save Changes
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  );
}

function NewText() {
  return (
    <div>
      <Jumbotron>
        <h1>Proposer un nouvel extrait de texte</h1>
        <p class="lead">
          Cette interface vous permet d’ajouter de nouveaux textes à
          Philosopher.fr. Avant d'ajouter un texte, prenez un moment pour
          vérifier qu'il n'est pas déjà présent dans la base.
        </p>
      </Jumbotron>

      <Tabs defaultActiveKey="input" id="uncontrolled-tab-example">
        <Tab eventKey="input" title="Saisie">
          <Card>
            <Card.Body>
              <NewTextEditionForm />
            </Card.Body>
          </Card>
        </Tab>
        <Tab eventKey="preview" title="Aperçu"></Tab>
      </Tabs>
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

function FatalError() {
  return <h1>Erreur catastrophique: Philosopher est en panne!…</h1>;
}

function UI() {
  // Base queries.  They'll get stored as this components's state.
  const notionsQuery = useQuery("progNotions", API.fetchProgNotions, {
    staleTime: 60 * 60 * 1000,
  });
  const reperesQuery = useQuery("progReperes", API.fetchProgReperes, {
    staleTime: 60 * 60 * 1000,
  });
  const hlpQuery = useQuery("progHLP", API.fetchProgHLP, {
    staleTime: 60 * 60 * 1000,
  });

  if (notionsQuery.isLoading || reperesQuery.isLoading || hlpQuery.isLoading) {
    return <h1>Chargement du programme officiel…</h1>;
  }
  if (notionsQuery.isError || reperesQuery.isError || hlpQuery.isError) {
    return <FatalError />;
  }

  return (
    <ProgrammeContext.Provider
      value={{
        notions: notionsQuery.data,
        reperes: reperesQuery.data,
        hlp: hlpQuery.data,
      }}
    >
      <Router>
        <AppHeader />

        <Container>
          <Switch>
            <Route exact path="/texte/nouveau">
              <NewText />
            </Route>
            <Route exact path="/">
              <Home />
            </Route>
            <Route path="*">
              <NotFound />
            </Route>
          </Switch>
        </Container>
        <br />
        <Navbar variant="dark" bg="dark" sticky="bottom">
          <p class="text-muted text-xs">
            Propulsé par <a href="https://github.com/thblt/theuth">Theuth</a>,
            un logiciel libre écrit en Rust. Crédits. Mentions légales.
          </p>
        </Navbar>
      </Router>
    </ProgrammeContext.Provider>
  );
}

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <UI />
    </QueryClientProvider>
  );
}
