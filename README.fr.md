# COMMENT UTILISER CE BOT

[Installer Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)     

Installer les dépendances :     
`$_> sudo apt install build-essential`     
`$_> sudo apt install librust-openssl-dev`     

Télécharger les sources du projet :     
`$_> git clone https://github.com/RevolioClockberg/RSS-Bot-Telegram.git`     

Compiler le projet en mode release :     
`$_> cd ./RSS-Bot-Telegram`     
`$_> cargo build --release`     

Lancer le bot avec le token et le userID, avec la redirection vers le fichier de log.      
`$_> ./target/release/RSS-Bot-Telegram BOT_TOKEN USER_ID > ./bot.log &`     


&nbsp;


# COMMENT FONCTIONNE CE BOT
Le Bot se lance en prenant en paramètre le token (obtenu à la création d'un bot avec le @BotFather) et votre UserID (obtenu avec @userinfobot).      
Lorsqu'il est lancé, il peut étre administrer depuis le chat privé (ouvert entre le Bot et vous). Il faut ajouter le Bot dans un channel Telegram et le mettre administrateur pour commencer à recevoir les infos des flux RSS.     
Une fois dans le channel, dès que le bot reçoit la commande "/start" provenant de ce channel, il va automatiquement checker la 
liste des flux RSS du fichier "feeds.txt" et commencer à envoyer les mises à jour de ces flux (s'il y en a), au fur et à mesure des publications.     
Par souci d'optimisation le Bot check les flux RSS toutes les 10 minutes et envoi les mises à jour à une minute d'intervale, pour ne pas flooder le channel.      

Le chat privé que vous obtenez à la création du bot va maintenant servir à administrer le Bot. Dans ce dernier, la commande "/start" vous donne les commandes que le bot peut exécuter.      
Attention ces commandes ne fonctionnent que dans le chat privé entre le Bot et l'utilisateur à qui apartient le UserID fournit au lancement du Bot.     
Le channel rejoint par le Bot n'est utilisé que pour envoyer les dernières mises à jour des flux RSS, mais le chat privé va permettre d'ajouter, d'enlever et de lister les flux RSS suivis.     

# Traduction
**Autres Langues:** [English](README.md)