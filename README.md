# Open Shop Protocol (OSP)

**Publish your products anywhere. Discover them everywhere.**

Open Shop Protocol (OSP) est un **standard ouvert** permettant de publier et découvrir des produits numériques sur le web.

Il introduit une **couche ouverte de commerce** permettant à n’importe quel créateur de publier ses produits depuis son propre site tout en restant découvrable via des index et moteurs compatibles.

OSP vise à casser les silos des plateformes fermées en permettant aux boutiques indépendantes d’être **interopérables et indexables**.

---

# Vision

Aujourd’hui, la majorité du commerce numérique est fragmentée :

- Shopify
- Gumroad
- marketplaces
- boutiques indépendantes
- sites personnels

Chaque boutique fonctionne comme un **silo isolé**.

Open Shop Protocol introduit une **couche standardisée** permettant :

- la publication universelle de produits
- la découverte distribuée
- l’interopérabilité entre plateformes

L’objectif est de créer l’équivalent de :

- RSS pour les blogs
- OpenGraph pour les previews
- ActivityPub pour les réseaux sociaux

mais pour **les produits numériques**.

---

# Principe fondamental

Chaque créateur peut publier ses produits via un format standardisé.

Ces produits peuvent ensuite être :

- indexés
- comparés
- recommandés
- affichés
- analysés

par n’importe quel service compatible OSP.

---

# Architecture

Le système se compose de **quatre éléments principaux**.

## 1. OSP Spec

La **spécification du protocole**.

Elle définit les endpoints et structures de données standard.

Exemples :

```
/.well-known/osp
/shop.json
/products.json
```

Ces fichiers décrivent :

- le shop
- les produits
- les métadonnées

---

## 2. OSP Node

Le **node OSP** est l’implémentation principale.

Il s’agit d’un **exécutable autonome** qui permet à un créateur de lancer sa boutique.

Le node inclut :

- serveur HTTP léger
- base locale
- panel admin
- frontend public
- endpoints OSP

Un utilisateur peut simplement :

1. télécharger l’exécutable
2. le lancer
3. configurer son shop
4. publier ses produits

---

## 3. OSP Index

Un index compatible OSP peut :

- crawler les shops
- indexer les produits
- permettre la recherche
- afficher les boutiques

Ton site pourrait être **le premier index public OSP**.

---

## 4. OSP Hosting

Une option d’hébergement gérée peut être proposée pour simplifier l’utilisation.

Cela permet :

- déploiement instantané
- sous-domaines
- performances garanties
- maintenance simplifiée

L’utilisateur garde toujours la possibilité de **s’auto-héberger gratuitement**.

---

# Modes d’utilisation

OSP propose deux modes principaux.

## Self-hosted (gratuit)

Le créateur lance son node lui-même.

Il contrôle :

- l’hébergement
- les performances
- l’infrastructure

C’est le mode **le plus décentralisé**.

---

## Hosted (facilité)

Le créateur utilise une version hébergée.

Avantages :

- setup instantané
- pas d’infrastructure
- maintenance automatique
- performances garanties

La simplicité devient le service payant.

---

# Philosophie

OSP repose sur trois principes fondamentaux.

## Ouverture

Le protocole est open source et librement implémentable.

## Interopérabilité

Toute plateforme peut publier ou indexer des produits.

## Indépendance

Les créateurs gardent le contrôle de leur boutique et de leurs revenus.

---

# Positionnement

OSP n’est **pas une marketplace**.

Ce n’est **pas une plateforme de vente centralisée**.

C’est une **infrastructure ouverte** qui connecte les boutiques du web.

---

# Modèle économique

Le protocole est **gratuit et ouvert**.

La monétisation peut se faire via :

- hébergement géré
- index premium
- analytics
- promotion produits
- API de discovery

---

# Licence

Le protocole est distribué sous :

**Apache License 2.0**

Cela permet :

- usage commercial
- adoption large
- contributions ouvertes

---

# Résumé

Open Shop Protocol est un standard ouvert permettant aux créateurs de publier leurs produits depuis n’importe quel site tout en restant découvrables à travers un réseau d’index compatibles.

---

# Roadmap

## Phase 1 — Protocole minimal (MVP)

Objectif : définir le cœur du protocole.

Features :

- spécification `shop.json`
- spécification `products.json`
- endpoint `/.well-known/osp`
- schéma JSON des produits
- documentation du protocole

---

## Phase 2 — OSP Node

Objectif : permettre à n’importe qui de lancer un shop.

Features :

- exécutable standalone
- serveur HTTP léger
- base de données locale
- panel admin
- frontend public
- export OSP automatique

---

## Phase 3 — Crawler

Objectif : découvrir les shops OSP.

Features :

- crawler OSP
- découverte via `.well-known`
- lecture des manifests
- indexation produits

---

## Phase 4 — Index public

Objectif : créer la première expérience utilisateur.

Features :

- moteur de recherche
- pages produits
- pages boutiques
- catégories
- tags

---

## Phase 5 — Widgets

Objectif : faciliter l’intégration.

Features :

- script JS pour afficher un produit
- script JS pour afficher un catalogue
- composants frontend

---

## Phase 6 — Plugins CMS

Objectif : faciliter l’adoption.

Plugins pour :

- WordPress
- Shopify
- CMS headless

Ces plugins exportent automatiquement les manifests OSP.

---

## Phase 7 — API

Objectif : ouvrir l’écosystème.

Endpoints :

```
/api/products
/api/shops
/api/search
```

Utilisation :

- marketplaces
- comparateurs
- apps
- IA

---

## Phase 8 — Ranking

Objectif : améliorer la découverte.

Features :

- popularité
- récence
- engagement
- qualité des shops

---

## Phase 9 — Product Graph

Objectif : créer des relations entre produits.

Exemples :

- produits similaires
- bundles
- dépendances
- collections

Cela permet de construire un **graph de produits interconnectés**.

---

# Vision long terme

Créer une **couche ouverte du commerce web** où :

- les créateurs contrôlent leurs boutiques
- les produits sont interopérables
- la découverte est distribuée
- l’infrastructure reste ouverte.
