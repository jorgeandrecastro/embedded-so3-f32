// Copyright (C) 2026 Jorge Andre Castro
//
// Ce programme est un logiciel libre : vous pouvez le redistribuer et/ou le modifier
// selon les termes de la Licence Publique Générale GNU telle que publiée par la
// Free Software Foundation, soit la version 2 de la licence, soit (à votre convention)
// n'importe quelle version ultérieure.

//! # embedded-vector
//!
//! Vecteurs `f32` 2D, 3D et 4D pour systèmes embarqués `no_std`.
//!
//! Sans dépendance standard, sans `unsafe`, sans allocation.
//! Utilise [`embedded_f32_sqrt`] pour le calcul de norme.
//!
//! ## Exemple rapide
//!
//! ```rust
//! use embedded_vector::{Vec2, Vec3, Vec4};
//!
//! // Arithmétique
//! let a = Vec3::new(1.0, 2.0, 3.0);
//! let b = Vec3::new(4.0, 5.0, 6.0);
//! let c = a + b;                          // Vec3 [5, 7, 9]
//!
//! // Produit scalaire et produit vectoriel
//! let d = a.dot(&b);                      // 32.0
//! let e = a.cross(&b);                    // Vec3 [-3, 6, -3]
//!
//! // Norme et normalisation
//! let n = a.norm().unwrap();              // √14 ≈ 3.742
//! let u = a.normalize().unwrap();         // vecteur unitaire
//!
//! // Opérateurs
//! let f = a * 2.0;                        // Vec3 [2, 4, 6]
//! let g = -b;                             // Vec3 [-4, -5, -6]
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use embedded_f32_sqrt::sqrt;

// ─────────────────────────────────────────────────────────────
//  Erreurs
// ─────────────────────────────────────────────────────────────

/// Erreurs retournées par les opérations sur les vecteurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VecError {
    /// Norme nulle ou trop proche de zéro : normalisation impossible.
    ZeroNorm,
    /// Une valeur `NaN` ou infinie a été détectée.
    NonFiniteValue,
}

// ─────────────────────────────────────────────────────────────
//  Vec2
// ─────────────────────────────────────────────────────────────

/// Vecteur 2D en `f32`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    /// Composante X.
    pub x: f32,
    /// Composante Y.
    pub y: f32,
}

impl Vec2 {
    /// Crée un nouveau vecteur 2D.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Vecteur nul `[0, 0]`.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Vecteur unité X `[1, 0]`.
    pub const X: Self = Self::new(1.0, 0.0);

    /// Vecteur unité Y `[0, 1]`.
    pub const Y: Self = Self::new(0.0, 1.0);

    /// Retourne les composantes sous la forme `[x, y]`.
    #[inline]
    pub fn as_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    /// Crée un vecteur depuis un tableau `[x, y]`.
    #[inline]
    pub const fn from_array(v: [f32; 2]) -> Self {
        Self::new(v[0], v[1])
    }

    /// Produit scalaire : `self · rhs = x₁x₂ + y₁y₂`.
    #[inline]
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Norme au carré : `x² + y²`. Infaillible.
    #[inline]
    pub fn norm_sq(&self) -> f32 {
        self.dot(self)
    }

    /// Norme euclidienne via [`embedded_f32_sqrt`].
    ///
    /// Retourne [`VecError::NonFiniteValue`] si un composant est `NaN` ou infini.
    pub fn norm(&self) -> Result<f32, VecError> {
        let sq = self.norm_sq();
        if !sq.is_finite() {
            return Err(VecError::NonFiniteValue);
        }
        sqrt(sq).map_err(|_| VecError::ZeroNorm)
    }

    /// Vecteur unitaire dans la même direction.
    ///
    /// Retourne [`VecError::ZeroNorm`] si la norme est nulle.
    /// Retourne [`VecError::NonFiniteValue`] si un composant est invalide.
    pub fn normalize(&self) -> Result<Self, VecError> {
        let n = self.norm()?;
        if n < 1e-10 {
            return Err(VecError::ZeroNorm);
        }
        Ok(Self::new(self.x / n, self.y / n))
    }

    /// Distance entre deux vecteurs.
    pub fn distance(&self, rhs: &Self) -> Result<f32, VecError> {
        (*self - *rhs).norm()
    }

    /// Interpolation linéaire : `self + t * (rhs - self)`, `t ∈ [0, 1]`.
    #[inline]
    pub fn lerp(&self, rhs: &Self, t: f32) -> Self {
        Self::new(
            self.x + t * (rhs.x - self.x),
            self.y + t * (rhs.y - self.y),
        )
    }

    /// Multiplication terme à terme (produit de Hadamard).
    #[inline]
    pub fn hadamard(&self, rhs: &Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }

    /// Retourne `true` si tous les composants sont finis (non NaN, non infini).
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl core::ops::Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self { Self::new(self.x + rhs.x, self.y + rhs.y) }
}
impl core::ops::Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self { Self::new(self.x - rhs.x, self.y - rhs.y) }
}
impl core::ops::Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, s: f32) -> Self { Self::new(self.x * s, self.y * s) }
}
impl core::ops::Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, s: f32) -> Self { Self::new(self.x / s, self.y / s) }
}
impl core::ops::Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self { Self::new(-self.x, -self.y) }
}
impl core::ops::AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.x += rhs.x; self.y += rhs.y; }
}
impl core::ops::SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.x -= rhs.x; self.y -= rhs.y; }
}
impl core::ops::MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, s: f32) { self.x *= s; self.y *= s; }
}

// ─────────────────────────────────────────────────────────────
//  Vec3
// ─────────────────────────────────────────────────────────────

/// Vecteur 3D en `f32`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// Composante X.
    pub x: f32,
    /// Composante Y.
    pub y: f32,
    /// Composante Z.
    pub z: f32,
}

impl Vec3 {
    /// Crée un nouveau vecteur 3D.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Vecteur nul `[0, 0, 0]`.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    /// Vecteur unité X `[1, 0, 0]`.
    pub const X: Self = Self::new(1.0, 0.0, 0.0);

    /// Vecteur unité Y `[0, 1, 0]`.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);

    /// Vecteur unité Z `[0, 0, 1]`.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    /// Retourne les composantes sous la forme `[x, y, z]`.
    #[inline]
    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    /// Crée un vecteur depuis un tableau `[x, y, z]`.
    #[inline]
    pub const fn from_array(v: [f32; 3]) -> Self {
        Self::new(v[0], v[1], v[2])
    }

    /// Produit scalaire : `self · rhs = x₁x₂ + y₁y₂ + z₁z₂`.
    #[inline]
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Produit vectoriel : `self × rhs`.
    ///
    /// Le résultat est perpendiculaire aux deux opérandes (règle de la main droite).
    /// Propriété : `a × b = -(b × a)`.
    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// Norme au carré : `x² + y² + z²`. Infaillible.
    #[inline]
    pub fn norm_sq(&self) -> f32 {
        self.dot(self)
    }

    /// Norme euclidienne via [`embedded_f32_sqrt`].
    ///
    /// Retourne [`VecError::NonFiniteValue`] si un composant est `NaN` ou infini.
    pub fn norm(&self) -> Result<f32, VecError> {
        let sq = self.norm_sq();
        if !sq.is_finite() {
            return Err(VecError::NonFiniteValue);
        }
        sqrt(sq).map_err(|_| VecError::ZeroNorm)
    }

    /// Vecteur unitaire dans la même direction.
    ///
    /// Retourne [`VecError::ZeroNorm`] si la norme est nulle.
    /// Retourne [`VecError::NonFiniteValue`] si un composant est invalide.
    pub fn normalize(&self) -> Result<Self, VecError> {
        let n = self.norm()?;
        if n < 1e-10 {
            return Err(VecError::ZeroNorm);
        }
        Ok(Self::new(self.x / n, self.y / n, self.z / n))
    }

    /// Distance euclidienne entre deux vecteurs.
    pub fn distance(&self, rhs: &Self) -> Result<f32, VecError> {
        (*self - *rhs).norm()
    }

    /// Interpolation linéaire : `self + t * (rhs - self)`, `t ∈ [0, 1]`.
    #[inline]
    pub fn lerp(&self, rhs: &Self, t: f32) -> Self {
        Self::new(
            self.x + t * (rhs.x - self.x),
            self.y + t * (rhs.y - self.y),
            self.z + t * (rhs.z - self.z),
        )
    }

    /// Multiplication terme à terme (produit de Hadamard).
    #[inline]
    pub fn hadamard(&self, rhs: &Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    /// Projection de `self` sur `rhs` : `(self·rhs / |rhs|²) * rhs`.
    ///
    /// Retourne [`VecError::ZeroNorm`] si `rhs` est le vecteur nul.
    pub fn project_onto(&self, rhs: &Self) -> Result<Self, VecError> {
        let denom = rhs.norm_sq();
        if denom < 1e-20 {
            return Err(VecError::ZeroNorm);
        }
        Ok(*rhs * (self.dot(rhs) / denom))
    }

    /// Composante de `self` perpendiculaire à `rhs` : `self - proj(self, rhs)`.
    pub fn reject_from(&self, rhs: &Self) -> Result<Self, VecError> {
        Ok(*self - self.project_onto(rhs)?)
    }

    /// Retourne `true` si tous les composants sont finis (non NaN, non infini).
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl core::ops::Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self { Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z) }
}
impl core::ops::Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self { Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z) }
}
impl core::ops::Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, s: f32) -> Self { Self::new(self.x * s, self.y * s, self.z * s) }
}
impl core::ops::Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, s: f32) -> Self { Self::new(self.x / s, self.y / s, self.z / s) }
}
impl core::ops::Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z) }
}
impl core::ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.x += rhs.x; self.y += rhs.y; self.z += rhs.z; }
}
impl core::ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z; }
}
impl core::ops::MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, s: f32) { self.x *= s; self.y *= s; self.z *= s; }
}

// ─────────────────────────────────────────────────────────────
//  Vec4
// ─────────────────────────────────────────────────────────────

/// Vecteur 4D en `f32`.
///
/// Utile pour les coordonnées homogènes (`w=1` pour un point, `w=0` pour
/// un vecteur de direction) et les couleurs RGBA.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    /// Composante X.
    pub x: f32,
    /// Composante Y.
    pub y: f32,
    /// Composante Z.
    pub z: f32,
    /// Composante W.
    pub w: f32,
}

impl Vec4 {
    /// Crée un nouveau vecteur 4D.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Vecteur nul `[0, 0, 0, 0]`.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// Retourne les composantes sous la forme `[x, y, z, w]`.
    #[inline]
    pub fn as_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Crée un vecteur depuis un tableau `[x, y, z, w]`.
    #[inline]
    pub const fn from_array(v: [f32; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }

    /// Construit un `Vec4` depuis un `Vec3` en ajoutant `w`.
    ///
    /// Convention : `w=1.0` pour un point, `w=0.0` pour une direction.
    #[inline]
    pub const fn from_vec3(v: Vec3, w: f32) -> Self {
        Self::new(v.x, v.y, v.z, w)
    }

    /// Extrait les composantes XYZ comme `Vec3`, en ignorant `w`.
    #[inline]
    pub const fn xyz(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Produit scalaire 4D : `x₁x₂ + y₁y₂ + z₁z₂ + w₁w₂`.
    #[inline]
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    /// Norme au carré. Infaillible.
    #[inline]
    pub fn norm_sq(&self) -> f32 {
        self.dot(self)
    }

    /// Norme euclidienne via [`embedded_f32_sqrt`].
    pub fn norm(&self) -> Result<f32, VecError> {
        let sq = self.norm_sq();
        if !sq.is_finite() {
            return Err(VecError::NonFiniteValue);
        }
        sqrt(sq).map_err(|_| VecError::ZeroNorm)
    }

    /// Vecteur unitaire dans la même direction.
    pub fn normalize(&self) -> Result<Self, VecError> {
        let n = self.norm()?;
        if n < 1e-10 {
            return Err(VecError::ZeroNorm);
        }
        Ok(Self::new(self.x / n, self.y / n, self.z / n, self.w / n))
    }

    /// Interpolation linéaire.
    #[inline]
    pub fn lerp(&self, rhs: &Self, t: f32) -> Self {
        Self::new(
            self.x + t * (rhs.x - self.x),
            self.y + t * (rhs.y - self.y),
            self.z + t * (rhs.z - self.z),
            self.w + t * (rhs.w - self.w),
        )
    }

    /// Multiplication terme à terme (produit de Hadamard).
    #[inline]
    pub fn hadamard(&self, rhs: &Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
    }

    /// Retourne `true` si tous les composants sont finis.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
            && self.z.is_finite() && self.w.is_finite()
    }
}

impl core::ops::Add for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}
impl core::ops::Sub for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}
impl core::ops::Mul<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }
}
impl core::ops::Div<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, s: f32) -> Self {
        Self::new(self.x / s, self.y / s, self.z / s, self.w / s)
    }
}
impl core::ops::Neg for Vec4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z, -self.w) }
}
impl core::ops::AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z; self.w += rhs.w;
    }
}
impl core::ops::SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z; self.w -= rhs.w;
    }
}
impl core::ops::MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, s: f32) {
        self.x *= s; self.y *= s; self.z *= s; self.w *= s;
    }
}

// ─────────────────────────────────────────────────────────────
//  Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    //  Vec2

    #[test]
    fn v2_constants() {
        assert_eq!(Vec2::ZERO.as_array(), [0.0, 0.0]);
        assert_eq!(Vec2::X.as_array(), [1.0, 0.0]);
        assert_eq!(Vec2::Y.as_array(), [0.0, 1.0]);
    }

    #[test]
    fn v2_dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert!((a.dot(&b) - 11.0).abs() < 1e-6);
    }

    #[test]
    fn v2_norm() {
        // |[3,4]| = 5
        assert!((Vec2::new(3.0, 4.0).norm().unwrap() - 5.0).abs() < 1e-5);
    }

    #[test]
    fn v2_normalize() {
        let u = Vec2::new(3.0, 0.0).normalize().unwrap();
        assert!((u.x - 1.0).abs() < 1e-6);
        assert!(u.y.abs() < 1e-6);
        assert!((u.norm().unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn v2_normalize_zero() {
        assert_eq!(Vec2::ZERO.normalize(), Err(VecError::ZeroNorm));
    }

    #[test]
    fn v2_distance() {
        assert!((Vec2::ZERO.distance(&Vec2::new(3.0, 4.0)).unwrap() - 5.0).abs() < 1e-5);
    }

    #[test]
    fn v2_lerp() {
        let m = Vec2::ZERO.lerp(&Vec2::new(2.0, 4.0), 0.5);
        assert!((m.x - 1.0).abs() < 1e-6);
        assert!((m.y - 2.0).abs() < 1e-6);
    }

    #[test]
    fn v2_hadamard() {
        let h = Vec2::new(2.0, 3.0).hadamard(&Vec2::new(4.0, 5.0));
        assert!((h.x - 8.0).abs() < 1e-6);
        assert!((h.y - 15.0).abs() < 1e-6);
    }

    #[test]
    fn v2_ops() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert!((( a + b).x - 4.0).abs() < 1e-6);
        assert!(((b - a).x - 2.0).abs() < 1e-6);
        assert!((( a * 3.0).y - 6.0).abs() < 1e-6);
        assert!((( b / 2.0).x - 1.5).abs() < 1e-6);
        assert!(((-a).x + 1.0).abs() < 1e-6);
    }

    #[test]
    fn v2_assign_ops() {
        let mut v = Vec2::new(1.0, 1.0);
        v += Vec2::new(2.0, 3.0);
        assert!((v.x - 3.0).abs() < 1e-6);
        v -= Vec2::new(1.0, 0.0);
        assert!((v.x - 2.0).abs() < 1e-6);
        v *= 2.0;
        assert!((v.x - 4.0).abs() < 1e-6);
    }

    #[test]
    fn v2_non_finite() {
        let v = Vec2::new(f32::NAN, 0.0);
        assert_eq!(v.norm(), Err(VecError::NonFiniteValue));
        assert!(!v.is_finite());
    }

    // Vec3

    #[test]
    fn v3_constants() {
        assert_eq!(Vec3::ZERO.as_array(), [0.0, 0.0, 0.0]);
        assert_eq!(Vec3::X.as_array(), [1.0, 0.0, 0.0]);
        assert_eq!(Vec3::Y.as_array(), [0.0, 1.0, 0.0]);
        assert_eq!(Vec3::Z.as_array(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn v3_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert!((a.dot(&b) - 32.0).abs() < 1e-6);
    }

    #[test]
    fn v3_cross_basis() {
        // X × Y = Z,  Y × Z = X,  Z × X = Y
        let z = Vec3::X.cross(&Vec3::Y);
        assert!(z.x.abs() < 1e-6);
        assert!(z.y.abs() < 1e-6);
        assert!((z.z - 1.0).abs() < 1e-6);

        let x = Vec3::Y.cross(&Vec3::Z);
        assert!((x.x - 1.0).abs() < 1e-6);

        let y = Vec3::Z.cross(&Vec3::X);
        assert!((y.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn v3_cross_anticommutative() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let axb = a.cross(&b);
        let bxa = b.cross(&a);
        // a × b = -(b × a)
        assert!((axb.x + bxa.x).abs() < 1e-6);
        assert!((axb.y + bxa.y).abs() < 1e-6);
        assert!((axb.z + bxa.z).abs() < 1e-6);
    }

    #[test]
    fn v3_cross_perpendicular() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = a.cross(&b);
        assert!(a.dot(&c).abs() < 1e-5);
        assert!(b.dot(&c).abs() < 1e-5);
    }

    #[test]
    fn v3_norm() {
        // |[1,2,2]| = 3
        assert!((Vec3::new(1.0, 2.0, 2.0).norm().unwrap() - 3.0).abs() < 1e-5);
    }

    #[test]
    fn v3_normalize() {
        let u = Vec3::new(0.0, 0.0, 5.0).normalize().unwrap();
        assert!(u.x.abs() < 1e-6);
        assert!(u.y.abs() < 1e-6);
        assert!((u.z - 1.0).abs() < 1e-6);
        assert!((u.norm().unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn v3_normalize_zero() {
        assert_eq!(Vec3::ZERO.normalize(), Err(VecError::ZeroNorm));
    }

    #[test]
    fn v3_distance() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(4.0, 0.0, 0.0);
        assert!((a.distance(&b).unwrap() - 3.0).abs() < 1e-5);
    }

    #[test]
    fn v3_lerp() {
        let m = Vec3::ZERO.lerp(&Vec3::new(2.0, 4.0, 6.0), 0.5);
        assert!((m.x - 1.0).abs() < 1e-6);
        assert!((m.y - 2.0).abs() < 1e-6);
        assert!((m.z - 3.0).abs() < 1e-6);
    }

    #[test]
    fn v3_project_onto() {
        let p = Vec3::new(3.0, 4.0, 0.0).project_onto(&Vec3::X).unwrap();
        assert!((p.x - 3.0).abs() < 1e-6);
        assert!(p.y.abs() < 1e-6);
    }

    #[test]
    fn v3_project_onto_zero() {
        assert_eq!(Vec3::X.project_onto(&Vec3::ZERO), Err(VecError::ZeroNorm));
    }

    #[test]
    fn v3_reject_from() {
        // [3,4,0] rejeté de X → [0,4,0]
        let r = Vec3::new(3.0, 4.0, 0.0).reject_from(&Vec3::X).unwrap();
        assert!(r.x.abs() < 1e-6);
        assert!((r.y - 4.0).abs() < 1e-6);
        assert!(r.z.abs() < 1e-6);
    }

    #[test]
    fn v3_hadamard() {
        let h = Vec3::new(1.0, 2.0, 3.0).hadamard(&Vec3::new(4.0, 5.0, 6.0));
        assert!((h.x - 4.0).abs() < 1e-6);
        assert!((h.y - 10.0).abs() < 1e-6);
        assert!((h.z - 18.0).abs() < 1e-6);
    }

    #[test]
    fn v3_ops() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert!(((a + b).x - 5.0).abs() < 1e-6);
        assert!(((b - a).x - 3.0).abs() < 1e-6);
        assert!(((a * 2.0).z - 6.0).abs() < 1e-6);
        assert!(((b / 2.0).x - 2.0).abs() < 1e-6);
        assert!(((-a).x + 1.0).abs() < 1e-6);
    }

    #[test]
    fn v3_assign_ops() {
        let mut v = Vec3::new(1.0, 1.0, 1.0);
        v += Vec3::new(1.0, 2.0, 3.0);
        assert!((v.z - 4.0).abs() < 1e-6);
        v -= Vec3::new(0.0, 0.0, 1.0);
        assert!((v.z - 3.0).abs() < 1e-6);
        v *= 2.0;
        assert!((v.z - 6.0).abs() < 1e-6);
    }

    #[test]
    fn v3_non_finite() {
        let v = Vec3::new(0.0, f32::INFINITY, 0.0);
        assert_eq!(v.norm(), Err(VecError::NonFiniteValue));
        assert!(!v.is_finite());
    }

    #[test]
    fn v3_from_array_roundtrip() {
        let arr = [1.0_f32, 2.0, 3.0];
        assert_eq!(Vec3::from_array(arr).as_array(), arr);
    }

    // Vec4 

    #[test]
    fn v4_from_vec3() {
        let v4 = Vec4::from_vec3(Vec3::new(1.0, 2.0, 3.0), 1.0);
        assert!((v4.w - 1.0).abs() < 1e-6);
        let back = v4.xyz();
        assert!((back.x - 1.0).abs() < 1e-6);
        assert!((back.z - 3.0).abs() < 1e-6);
    }

    #[test]
    fn v4_dot() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert!((a.dot(&Vec4::new(1.0, 0.0, 0.0, 0.0)) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn v4_norm() {
        assert!((Vec4::new(1.0, 0.0, 0.0, 0.0).norm().unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn v4_normalize() {
        let u = Vec4::new(2.0, 0.0, 0.0, 0.0).normalize().unwrap();
        assert!((u.x - 1.0).abs() < 1e-6);
        assert!((u.norm().unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn v4_lerp() {
        let m = Vec4::ZERO.lerp(&Vec4::new(2.0, 4.0, 6.0, 8.0), 0.5);
        assert!((m.w - 4.0).abs() < 1e-6);
    }

    #[test]
    fn v4_ops() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(1.0, 1.0, 1.0, 1.0);
        assert!(((a + b).w - 5.0).abs() < 1e-6);
        assert!(((a * 2.0).w - 8.0).abs() < 1e-6);
        assert!(((-a).x + 1.0).abs() < 1e-6);
    }

    #[test]
    fn v4_non_finite() {
        assert!(!Vec4::new(0.0, 0.0, 0.0, f32::NAN).is_finite());
    }

    #[test]
    fn v4_hadamard() {
        let h = Vec4::new(1.0, 2.0, 3.0, 4.0).hadamard(&Vec4::new(2.0, 2.0, 2.0, 2.0));
        assert!((h.w - 8.0).abs() < 1e-6);
    }
}