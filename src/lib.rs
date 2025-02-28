// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use alloy_primitives::Address;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    prelude::*,
    storage::{StorageAddress, StorageMap, StorageString, StorageVec},
};

#[storage]
pub struct Gitfreedom {
    // Repository name
    name: StorageString,
    // Repository description
    description: StorageString,
    // Repository hash
    hash: StorageString,
    // Repository colaborators;
    colaborators: StorageVec<StorageAddress>,
}

#[storage]
#[entrypoint]
pub struct Contract {
    map: StorageMap<Address, StorageVec<Gitfreedom>>,
    owners: StorageVec<StorageAddress>,
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Contract {
    /// List all stored repositories.
    pub fn list_all(&self) -> (Vec<Address>, Vec<(String, String, String, Vec<Address>)>) {
        let own_len = self.owners.len();
        let mut gf_vec = Vec::with_capacity(own_len);
        let mut own_vec = Vec::with_capacity(own_len);
        for i in 0..own_len {
            let owner = self.owners.get(i).unwrap();
            let repos = self.map.get(owner);

            let repo_len = repos.len();
            for j in 0..repo_len {
                let repo = repos.get(j).unwrap();
                let col_len = repo.colaborators.len();
                let mut colaborators = Vec::with_capacity(col_len);
                for j in 0..col_len {
                    colaborators.push(repo.colaborators.get(j).unwrap());
                }
                own_vec.push(owner);
                gf_vec.push((
                    repo.name.get_string(),
                    repo.description.get_string(),
                    repo.hash.get_string(),
                    colaborators,
                ));
            }
        }

        (own_vec, gf_vec)
    }

    /// Add a new repository.
    pub fn add_repository(&mut self, name: String, hash: String) {
        // O owner é o chamador da função.
        let owner: Address = self.vm().msg_sender();

        // Insere no mapping associando o owner ao repositório.
        let mut repo_vec = self.map.setter(owner);
        let mut repo = repo_vec.grow();
        repo.name.set_str(name);
        repo.hash.set_str(hash);
        repo.description.set_str("Description");
        repo.colaborators.push(owner);

        // Verifica se o owner já consta no vetor owners.
        let mut exists = false;
        let own_len = self.owners.len();
        for i in 0..own_len {
            let existing_owner = self.owners.get(i).unwrap();
            if existing_owner == owner {
                exists = true;
                break;
            }
        }
        // Se não existir, adiciona o owner.
        if !exists {
            self.owners.push(owner);
        }
    }

    /// Get a repository by name and owner.
    pub fn get_repository(
        &self,
        name: String,
        owner: Address,
    ) -> (String, String, String, Vec<Address>) {
        // Obtém o vetor de repositórios associado ao owner.
        let repos = self.map.get(owner);
        let repo_len = repos.len();
        // Itera sobre os repositórios do owner.
        for i in 0..repo_len {
            let repo = repos.get(i).unwrap();
            // Compara o nome armazenado com o nome informado.
            if repo.name.get_string() == name {
                // Se encontrar, monta um vetor com os colaboradores.
                let col_len = repo.colaborators.len();
                let mut colaborators = Vec::with_capacity(col_len);
                for j in 0..col_len {
                    colaborators.push(repo.colaborators.get(j).unwrap());
                }
                return (
                    repo.name.get_string(),
                    repo.description.get_string(),
                    repo.hash.get_string(),
                    colaborators,
                );
            }
        }

        // Se não encontrar, retornar valores vazios.
        (
            String::from(""),
            String::from(""),
            String::from(""),
            Vec::new(),
        )
    }
}
