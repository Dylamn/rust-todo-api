# 🚀 Roadmap détaillée API TODO (avec Axum)

---

# 🧱 Phase 0 — Setup

## Objectif

Serveur HTTP minimal

### À faire

* `cargo new todo-api`
* ajouter les crates `axum` et `tokio`

---

## Implémenter

* Implémenter le ``Hello, world!`` de la documentation de `axum`.

---

# 🧩 Phase 1 — Routing & structure

## Objectif

Structure propre

---

## Arborescence

```text
src/
 ├── main.rs
 ├── routes/
 │    ├── mod.rs
 │    └── tasks.rs
 ├── handlers/
 ├── models/
 ├── state/
 └── error/
```

---

## À faire

* créer `/tasks`
* route GET

---

# 📦 Phase 2 — Modèle Task

## Objectif

Struct + sérialisation

> Reprendre la structure du projet CLI.

---

# 🧠 Phase 3 — State (IMPORTANT)

👉 Introduit :

```rust
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub tasks: Mutex<Vec<Task>>,
}
```

---

👉 Injecté dans Axum :

```rust
let state = Arc::new(AppState { tasks: Mutex::new(vec![]) });

let app = Router::new()
.route("/tasks", get(get_tasks))
.with_state(state);
```

---

# 🔧 Phase 4 — CRUD basique

## Endpoints

| Method | Route        | Description |
|--------|--------------|-------------|
| GET    | `/tasks`     | liste       |
| POST   | `/tasks`     | créer       |
| PATCH  | `/tasks/:id` | compléter   |
| DELETE | `/tasks/:id` | supprimer   |

---

## Concepts appris

* extractors (`Json`, `Path`)
* réponses HTTP
* mutation state

---

# ⚠️ Phase 5 — Gestion d’erreurs

👉 Créer un type d’erreur :

```rust
pub enum ApiError {
    NotFound,
    BadRequest,
}
```

👉 Implémenter `IntoResponse`

---

# 🧪 Phase 6 — Tests

## Types

* tests handlers
* tests routes

---

## Libs utiles

* `tower::ServiceExt`
* `http-body-util`

---

# 💾 Phase 7 — Persistance

## Option simple

* fichier JSON (réutilise ton code CLI 😏)

## Option avancée

👉 SQLx

* PostgreSQL
* migrations

---

# 🔐 Phase 8 — Auth (optionnel mais top)

* JWT
* middleware

---

# ⚙️ Phase 9 — Logging

👉 utiliser tracing

---

# 🌐 Phase 10 — Client CLI

👉 ton projet actuel devient :

```text
CLI → API HTTP → storage
```

---

# 🧠 Architecture finale cible

```text
[ CLI ]
   ↓ HTTP
[ API Axum ]
   ↓
[ Storage / DB ]
```

---

# 🎯 Bonus (niveau supérieur)

* pagination
* filtering
* OpenAPI (Swagger)
* rate limiting

---

# 🗺 Plan d’attaque concret

## Semaine 1

* Phase 0 → 3

## Semaine 2

* CRUD complet

## Semaine 3

* persistence + tests

---

# 🎯 Résumé

👉 Fais :

1. API Axum
2. CRUD mémoire
3. persistence
4. connecter le CLI
