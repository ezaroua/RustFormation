TP8 – Protocole Réseau Personnalisé:

1.Objectif:
Concevoir et implémenter un protocole réseau bas niveau sur TCP, permettant d'échanger des messages entre un client et un serveur.

2.Architecture:

proto_server/ : Serveur TCP capable de gérer plusieurs clients en parallèle.

proto_client/ : Client TCP qui envoie des messages en suivant un format personnalisé.

3.Lancer le TP:

    -coté serveur:
        cd proto_server
        cargo run

    -coté client:

        cd proto_client
        cargo run

4.Fonctionnalités:

    -Gestion manuelle de la sérialisation et désérialisation

    -Représentation binaire des messages

    -Support multi-clients

    -Codes d'opération pour login/message/quit

5.Compétences mises en oeuvre:

    -Programmation TCP bas niveau

    -Conception d’un protocole réseau

    -Gestion des états (connexion, déconnexion)

    -Traitement binaire (pas de serde)

TP9 – Serveur et Client WebSocket:

1.Objectif:

Implémenter un serveur WebSocket et un client WebSocket capables de communiquer en temps réel grâce au protocole WebSocket


2.Architecture

websocket_server/ : Serveur WebSocket basé sur tokio-tungstenite

websocket_client/ : Client WebSocket interactif

3.Technologies utilisées

    -Tokio
    -tokio-tungstenite
    -futures-util
    -tungstenite
    -chrono (pour timestamp)

4.Fonctionnalités

-Connexion WebSocket complète (handshake)

-Gestion de plusieurs clients simultanés

-Communication full-duplex

-Réponse automatique du serveur à chaque message

-Envoi automatique de messages (ping) toutes les 5 secondes

-Horodatage lisible avec chrono

5.Lancer le TP:

-Serveur :
    cd websocket_server
    cargo run

-Client :    
    cd websocket_client
    cargo run


6.Compétences mises enoeuvre

-Utilisation d’un protocole existant (WebSocket)

-Programmation asynchrone avec Tokio

-Gestion de split(), SinkExt, StreamExt

-Envoi et réception de messages texte