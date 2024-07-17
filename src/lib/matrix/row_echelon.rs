use crate::matrix::model::Matrix;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;

impl<K> Matrix<K>
where K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + PartialEq + Copy + Default {
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut result: Matrix<K> = self.clone();
        let mut pivot_index: usize = 0;

        for r in 0..self.row() {
            if pivot_index >= self.col() { // sortie à 3 sur une matrice 3x3
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
                        break;
                    }
                }
            }

            if pivot_index == self.col() { // sortie si plus de col à traiter
                break;
            }

            // échange r avec la ligne où le pivot non null a été trouvé
            result.data.swap(r, pivot_row);

            // normalisation du pivot : Divise la ligne par la valeur du pivot pour le set à 1
            let pivot_value: K = result.data[r][pivot_index];
            if pivot_value != K::default() {
                for j in 0..self.col() {
                    result.data[r][j] = result.data[r][j] / pivot_value;
                }
            }

            // mettre valeurs de la colonne à 0
            for i in 0..self.row() {
                if i != r && result.data[i][pivot_index] != K::default() { // exclure r et valeur nulle
                    let factor: K = result.data[i][pivot_index];
                    for k in 0..self.col() { // équivaut à Li <- Li - KLr
                        result.data[i][k] = result.data[i][k] - factor * result.data[r][k];
                    }
                }
            }
            pivot_index += 1;
        }

        result
        }
    }