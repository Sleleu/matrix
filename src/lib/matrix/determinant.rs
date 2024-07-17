use crate::matrix::model::Matrix;
use std::ops::{Add, Sub, Mul, Div, Neg};

impl<K> Matrix<K>
where K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + PartialEq + Copy + Default + Neg<Output = K>
{
    pub fn determinant(&self) -> K {
        let mut result: Matrix<K> = self.clone();
        let mut pivot_index: usize = 0;
        let mut determinant: K = K::default();

        for r in 0..self.row() {
            if pivot_index >= self.col() {
                break;
            }
            // trouver un pivot qui est diff de 0
            let mut pivot_row: usize = r;
            while result.data[pivot_row][pivot_index] == K::default() {
                pivot_row += 1;
                if pivot_row == self.row() { // aucun pivot non null trouvé
                    pivot_row = r;
                    pivot_index += 1; // donc on passe à la col suivante
                    if pivot_index == self.col() {
                        determinant = K::default(); // si une ligne ou colonne ne contient que des 0, det(A) = 0
                        break;
                    }
                }
            }

            if pivot_index == self.col() { // sortie si plus de col à traiter
                break;
            }

            // échange r avec la ligne où le pivot non null a été trouvé, equivaut a Li <-> Lj, on inverse le determinant alors
            if r != pivot_row {
                result.data.swap(r, pivot_row);
                determinant = -determinant;
            }
            // mettre valeurs de la colonne à 0
            for i in 0..self.row() {
                if i > r && result.data[i][pivot_index] != K::default() { // exclure r et valeur nulle
                    let factor: K = result.data[i][pivot_index] / result.data[pivot_index][pivot_index];
                    for k in 0..self.col() { // équivaut à Li <- Li - KLr
                        result.data[i][k] = result.data[i][k] - factor * result.data[r][k];
                    }
                }
            }

            match r {
                0 => {determinant = result.data[r][r]},
                _ => {determinant = determinant * result.data[r][r]},
            }

            pivot_index += 1;
        }
        determinant
        }
}