pub mod one;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let resultat = 2 + 2;
        assert_eq!(resultat, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn failed_test() {
    //     panic!("failure MacFailFace")
    // }

    #[derive(Debug)]
    struct Rectangle {
        largeur: u32,
        hauteur: u32,
    }

    impl Rectangle {
        fn peut_contenir(&self, other: &Rectangle) -> bool {
            self.largeur > other.largeur && self.hauteur > other.hauteur
        }
    }

    #[test]
    fn un_grand_peut_contenir_un_petit() {
        let le_grand = Rectangle {
            largeur: 8,
            hauteur: 7,
        };
        let le_petit = Rectangle {
            largeur: 5,
            hauteur: 1,
        };

        assert!(le_grand.peut_contenir(&le_petit));
    }
}
