
CREATE TABLE sources (
  -- Sources of the texts, as a sort of very rudimentary
  -- bibliographical db.  The format is very primitive for now, it
  -- should improve over time.
  id SERIAL UNIQUE PRIMARY KEY ,
  author INT NULL,
  title VARCHAR NOT NULL,
  containing_title VARCHAR NULL,
  FOREIGN KEY (author) REFERENCES authors (id)
  );

CREATE TABLE texts_notions (
  -- Many to many relationships between texts and program entries.
  link_id SERIAL UNIQUE PRIMARY KEY , -- Diesel wants this.
  text_id SERIAL,
  notion_id SERIAL,
  UNIQUE (text_id, notion_id)
  FOREIGN KEY (text_id) REFERENCES texts (id),
  FOREIGN KEY (notion_id) REFERENCES notions_philo (id)
  );

CREATE TABLE notions_philo (
  id SERIAL UNIQUE PRIMARY KEY ,
  slug VARCHAR UNIQUE NOT NULL,
  name VARCHAR NOT NULL,
  le_name VARCHAR NOT NULL,
  techno INT -- Boolean
  );

CREATE TABLE parties_hlp (
  id SERIAL UNIQUE PRIMARY KEY ,
  slug VARCHAR UNIQUE NOT NULL,
  semestre SERIAL NOT NULL,
  partie SERIAL NOT NULL, -- On code le semestre entier comme partie 0.
  name VARCHAR NOT NULL
  );

CREATE TABLE reperes_philo (
  id SERIAL UNIQUE PRIMARY KEY ,
  slug VARCHAR UNIQUE NOT NULL,
  name VARCHAR NOT NULL
  );

INSERT INTO notions_philo
  (slug, name, le_name, techno)
VALUES
  ("art"         , "art"         , "l’art"         , 1),
  ("bonheur"     , "bonheur"     , "le bonheur"    , 0),
  ("conscience"  , "conscience"  , "la conscience" , 0),
  ("devoir"      , "devoir"      , "le devoir"     , 0),
  ("Etat"        , "État"        , "l’État"        , 0),
  ("inconscient" , "inconscient" , "l’inconscient" , 0),
  ("justice"     , "justice"     , "la justice"    , 1),
  ("langage"     , "langage"     , "le langage"    , 0),
  ("liberte"     , "liberté"     , "la liberté"    , 1),
  ("nature"      , "nature"      , "la nature"     , 1),
  ("raison"      , "raison"      , "la raison"     , 0),
  ("religion"    , "religion"    , "la religion"   , 1),
  ("science"     , "science"     , "la science"    , 0),
  ("technique"   , "technique"   , "la technique"  , 0),
  ("temps"       , "temps"       , "le temps"      , 0),
  ("travail"     , "travail"     , "le travail"    , 0),
  ("verite"      , "vérité"      , "la vérité"     , 1);

INSERT INTO parties_hlp
  (slug, semestre, partie, name)
VALUES
  ("parole"        , 1 , 0 , "Les pouvoirs de la parole"),
  ("art"           , 1 , 1 , "L’art de la parole"),
  ("autorite"      , 1 , 2 , "L’autorité de la parole"),
  ("seductions"    , 1 , 3 , "Les séductions de la parole"),
  ("monde"         , 2 , 0 , "Les représentations du monde"),
  ("decouverte"    , 2 , 1 , "Découverte du monde et pluralité des cultures"),
  ("decrire"       , 2 , 2 , "Décrire, figurer, imaginer"),
  ("animal"        , 2 , 3 , "L’homme et l’animal"),
  ("soi"           , 3 , 0 , "La recherche de soi"),
  ("education"     , 3 , 1 , "Éducation, transmission et émancipation"),
  ("sensibilite"   , 3 , 2 , "Les expressions de la sensibilité"),
  ("metamorphoses" , 3 , 3 , "Les métamorphoses du moi"),
  ("humanite"      , 4 , 0 , "L’Humanité en question"),
  ("ruptures"      , 4 , 1 , "Création, continuités et ruptures"),
  ("histoire"      , 4 , 2 , "Histoire et violence"),
  ("limites"       , 4 , 3 , "L’humain et ses limites");

INSERT INTO reperes_philo
  (slug, name)
VALUES
  ("absolu-relatif"                          , "Absolu/relatif"),
  ("abstrait-concret"                        , "Abstrait/concret"),
  ("en acte-en puissance"                    , "En acte/en puissance"),
  ("analyse-synthese"                        , "Analyse/synthèse"),
  ("concept-image-metaphore"                 , "Concept/image/métaphore"),
  ("contingent-necessaire"                   , "Contingent/nécessaire"),
  ("croire-savoir"                           , "Croire/savoir"),
  ("essentiel-accidentel"                    , "Essentiel/accidentel"),
  ("exemple-preuve"                          , "Exemple/preuve"),
  ("expliquer-comprendre"                    , "Expliquer/comprendre"),
  ("fait-droit"                              , "En fait/en droit"),
  ("formel-materiel"                         , "Formel/matériel"),
  ("genre-espece-individu"                   , "Genre/espèce/individu"),
  ("hypothese-consequence-conclusion"        , "Hypothèse/conséquence/conclusion"),
  ("ideal-reel"                              , "Idéal/réel"),
  ("identite-egalite-difference"             , "Identité/égalité/différence"),
  ("impossible-possible"                     , "Impossible/possible"),
  ("intuitif-discursif"                      , "Intuitif/discursif"),
  ("legal-legitime"                          , "Légal/légitime"),
  ("mediat-immediat"                         , "Médiat/immédiat"),
  ("objectif-subjectif-intersubjectif"       , "Objectif/subjectif/intersubjectif"),
  ("obligation-contrainte"                   , "Obligation/contrainte"),
  ("origine-fondement"                       , "Origine/fondement"),
  ("persuader-convaincre"                    , "Persuader/convaincre"),
  ("principe-cause-fin"                      , "Principe/cause/fin"),
  ("public-prive"                            , "Public/privé"),
  ("ressemblance-analogie"                   , "Ressemblance/analogie"),
  ("theorie-pratique"                        , "Théorie/pratique"),
  ("transcendant-immanent"                   , "Transcendant/immanent"),
  ("universel-general-particulier-singulier" , "Universel/général/particulier/singulier"),
  ("vrai-probable-certain."                  , "Vrai/probable/certain");
