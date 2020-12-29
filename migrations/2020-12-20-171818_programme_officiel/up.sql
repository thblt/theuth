-- Première étape: formaliser le programme officiel.  Ce premier état
-- de la base contient trois tables: les notions et repères de la
-- philo de tronc commun, et le programme de HLP (sur deux ans,
-- première et terminale).

CREATE TABLE prog_notions (
  id SERIAL UNIQUE PRIMARY KEY,
  slug TEXT UNIQUE NOT NULL,
  name TEXT NOT NULL,
  le_name TEXT NOT NULL,
  techno boolean NOT NULL
  );

CREATE TABLE prog_hlp(
  id SERIAL UNIQUE PRIMARY KEY ,
  slug TEXT UNIQUE NOT NULL,
  name TEXT NOT NULL, -- On code le semestre entier comme partie 0.
  semestre integer NOT NULL,
  partie integer NULL
  );

CREATE TABLE prog_reperes (
  id SERIAL UNIQUE PRIMARY KEY ,
  slug VARCHAR UNIQUE NOT NULL,
  name VARCHAR NOT NULL
  );

INSERT INTO prog_notions
  (slug, name, le_name, techno)
VALUES
  ('art'         , 'art'         , 'l’art'         , true),
  ('bonheur'     , 'bonheur'     , 'le bonheur'    , false),
  ('conscience'  , 'conscience'  , 'la conscience' , false),
  ('devoir'      , 'devoir'      , 'le devoir'     , false),
  ('Etat'        , 'État'        , 'l’État'        , false),
  ('inconscient' , 'inconscient' , 'l’inconscient' , false),
  ('justice'     , 'justice'     , 'la justice'    , true),
  ('langage'     , 'langage'     , 'le langage'    , false),
  ('liberte'     , 'liberté'     , 'la liberté'    , true),
  ('nature'      , 'nature'      , 'la nature'     , true),
  ('raison'      , 'raison'      , 'la raison'     , false),
  ('religion'    , 'religion'    , 'la religion'   , true),
  ('science'     , 'science'     , 'la science'    , false),
  ('technique'   , 'technique'   , 'la technique'  , false),
  ('temps'       , 'temps'       , 'le temps'      , false),
  ('travail'     , 'travail'     , 'le travail'    , false),
  ('verite'      , 'vérité'      , 'la vérité'     , true);

INSERT INTO prog_hlp
  (slug, semestre, partie, name)
VALUES
  ('parole'        , 1 , 0 , 'Les pouvoirs de la parole'),
  ('art'           , 1 , 1 , 'L’art de la parole'),
  ('autorite'      , 1 , 2 , 'L’autorité de la parole'),
  ('seductions'    , 1 , 3 , 'Les séductions de la parole'),
  ('monde'         , 2 , 0 , 'Les représentations du monde'),
  ('decouverte'    , 2 , 1 , 'Découverte du monde et pluralité des cultures'),
  ('decrire'       , 2 , 2 , 'Décrire, figurer, imaginer'),
  ('animal'        , 2 , 3 , 'L’homme et l’animal'),
  ('soi'           , 3 , 0 , 'La recherche de soi'),
  ('education'     , 3 , 1 , 'Éducation, transmission et émancipation'),
  ('sensibilite'   , 3 , 2 , 'Les expressions de la sensibilité'),
  ('metamorphoses' , 3 , 3 , 'Les métamorphoses du moi'),
  ('humanite'      , 4 , 0 , 'L’Humanité en question'),
  ('ruptures'      , 4 , 1 , 'Création, continuités et ruptures'),
  ('histoire'      , 4 , 2 , 'Histoire et violence'),
  ('limites'       , 4 , 3 , 'L’humain et ses limites');

INSERT INTO prog_reperes
  (slug, name)
VALUES
  ('absolu-relatif'                          , 'Absolu/relatif'),
  ('abstrait-concret'                        , 'Abstrait/concret'),
  ('en acte-en puissance'                    , 'En acte/en puissance'),
  ('analyse-synthese'                        , 'Analyse/synthèse'),
  ('concept-image-metaphore'                 , 'Concept/image/métaphore'),
  ('contingent-necessaire'                   , 'Contingent/nécessaire'),
  ('croire-savoir'                           , 'Croire/savoir'),
  ('essentiel-accidentel'                    , 'Essentiel/accidentel'),
  ('exemple-preuve'                          , 'Exemple/preuve'),
  ('expliquer-comprendre'                    , 'Expliquer/comprendre'),
  ('fait-droit'                              , 'En fait/en droit'),
  ('formel-materiel'                         , 'Formel/matériel'),
  ('genre-espece-individu'                   , 'Genre/espèce/individu'),
  ('hypothese-consequence-conclusion'        , 'Hypothèse/conséquence/conclusion'),
  ('ideal-reel'                              , 'Idéal/réel'),
  ('identite-egalite-difference'             , 'Identité/égalité/différence'),
  ('impossible-possible'                     , 'Impossible/possible'),
  ('intuitif-discursif'                      , 'Intuitif/discursif'),
  ('legal-legitime'                          , 'Légal/légitime'),
  ('mediat-immediat'                         , 'Médiat/immédiat'),
  ('objectif-subjectif-intersubjectif'       , 'Objectif/subjectif/intersubjectif'),
  ('obligation-contrainte'                   , 'Obligation/contrainte'),
  ('origine-fondement'                       , 'Origine/fondement'),
  ('persuader-convaincre'                    , 'Persuader/convaincre'),
  ('principe-cause-fin'                      , 'Principe/cause/fin'),
  ('public-prive'                            , 'Public/privé'),
  ('ressemblance-analogie'                   , 'Ressemblance/analogie'),
  ('theorie-pratique'                        , 'Théorie/pratique'),
  ('transcendant-immanent'                   , 'Transcendant/immanent'),
  ('universel-general-particulier-singulier' , 'Universel/général/particulier/singulier'),
  ('vrai-probable-certain.'                  , 'Vrai/probable/certain');
