# embedded-vector

Vecteurs `f32` 2D, 3D et 4D pour systèmes embarqués `no_std`.

[![Crates.io](https://img.shields.io/crates/v/embedded-vector)](https://crates.io/crates/embedded-vector)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/license-GPL--2.0--or--later-blue)](LICENSE)
[![no_std](https://img.shields.io/badge/no__std-✓-green)]()

---

## Caractéristiques

- **`no_std`** — aucune dépendance à la bibliothèque standard, compatible bare-metal
- **Sans `unsafe`** — `#![forbid(unsafe_code)]`
- **Sans allocation** — zéro heap, zéro `alloc`
- **Racine carrée embarquée** — norme calculée via [`embedded-f32-sqrt`](https://crates.io/crates/embedded-f32-sqrt) (Newton-Raphson, testé Cortex-M33/M4F)
- **Gestion d'erreurs explicite** — pas de panic, tout retourne un `Result`

---

## Dépendance

```toml
[dependencies]
embedded-vector = "0.1"
```

---

## Utilisation rapide

```rust
use embedded_vector::{Vec2, Vec3, Vec4};

// Arithmétique de base
let a = Vec3::new(1.0, 2.0, 3.0);
let b = Vec3::new(4.0, 5.0, 6.0);
let c = a + b;                          // Vec3 { x:5, y:7, z:9 }

// Produit scalaire
let d = a.dot(&b);                      // 32.0

// Produit vectoriel (Vec3 uniquement)
let e = a.cross(&b);                    // Vec3 { x:-3, y:6, z:-3 }

// Norme et normalisation
let n = a.norm().unwrap();              // √14 ≈ 3.742
let u = a.normalize().unwrap();         // vecteur unitaire

// Opérateurs scalaires
let f = a * 2.0;                        // Vec3 { x:2, y:4, z:6 }
let g = -b;                             // Vec3 { x:-4, y:-5, z:-6 }

// Interpolation linéaire
let m = a.lerp(&b, 0.5);

// Projection / rejet (Vec3)
let p = a.project_onto(&Vec3::X).unwrap();
let r = a.reject_from(&Vec3::X).unwrap();

// Coordonnées homogènes (Vec4)
let pt  = Vec4::from_vec3(a, 1.0);     // point
let dir = Vec4::from_vec3(a, 0.0);     // direction
let xyz = pt.xyz();                     // récupère le Vec3
```

---

## Types

### `Vec2`

| Méthode / Constante | Description |
|---|---|
| `Vec2::new(x, y)` | Construction |
| `Vec2::ZERO`, `Vec2::X`, `Vec2::Y` | Constantes |
| `dot(&rhs)` | Produit scalaire |
| `norm_sq()` | Norme au carré (infaillible) |
| `norm()` | Norme euclidienne → `Result<f32, VecError>` |
| `normalize()` | Vecteur unitaire → `Result<Vec2, VecError>` |
| `distance(&rhs)` | Distance euclidienne |
| `lerp(&rhs, t)` | Interpolation linéaire |
| `hadamard(&rhs)` | Produit terme à terme |
| `is_finite()` | Détection NaN / infini |
| `as_array()`, `from_array([f32; 2])` | Conversion tableau |

### `Vec3`

Tout ce que `Vec2` expose, plus :

| Méthode | Description |
|---|---|
| `Vec3::Z` | Constante axe Z |
| `cross(&rhs)` | Produit vectoriel (règle main droite) |
| `project_onto(&rhs)` | Projection sur un vecteur |
| `reject_from(&rhs)` | Composante perpendiculaire |

### `Vec4`

| Méthode / Constante | Description |
|---|---|
| `Vec4::new(x, y, z, w)` | Construction |
| `Vec4::ZERO` | Constante |
| `from_vec3(v, w)` | Depuis un `Vec3` + composante `w` |
| `xyz()` | Extrait les composantes XYZ |
| `dot`, `norm`, `normalize`, `lerp`, `hadamard`, `is_finite` | Identiques aux types 2D/3D |

---

## Gestion d'erreurs

```rust
pub enum VecError {
    ZeroNorm,         // norme nulle, normalisation impossible
    NonFiniteValue,   // NaN ou infini détecté
}
```

Aucune opération ne produit de panic. Les opérations faillibles (`norm`, `normalize`, `distance`, `project_onto`, `reject_from`) retournent toutes un `Result`.

---

## Opérateurs surchargés

Tous les types implémentent :

`Add`, `Sub`, `Mul<f32>`, `Div<f32>`, `Neg`, `AddAssign`, `SubAssign`, `MulAssign<f32>`

---

## Sur la pertinence de `Vec4` en embarqué

`Vec4` est inclus principalement pour les cas suivants :

- **Coordonnées homogènes** — multiplication par une matrice 4×4 (transformation affine, projection perspective), où la distinction point (`w=1`) / direction (`w=0`) est nécessaire.
- **Couleurs RGBA** — représentation compacte sur un seul registre SIMD sur les cœurs qui en disposent (Cortex-M33 avec Helium, DSP avec SIMD 128 bits).
- **Quaternions** — stockage d'une rotation en `(x, y, z, w)` sans struct dédiée.

Si votre projet n'utilise aucun de ces cas, `Vec4` n'ajoute aucune overhead au binaire final (Rust ne compile que le code utilisé). Il ne coûte rien à garder dans la crate.

---

## Licence

GPL-2.0-or-later — voir [LICENSE](LICENSE).

Copyright © 2026 Jorge Andre Castro