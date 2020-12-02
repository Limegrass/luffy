use serde_json;
use std::fs;
use trello_git_webhook::gitea::*;

#[test]
fn serde_derive_correctly_deserializes() {
    let data = fs::read_to_string("tests/gitea/sample_response.json").expect("Unable to read file");
    let parsed: Response = serde_json::from_str(&data).expect("Malformed JSON");
    let expected = Response {
        r#ref: "refs/heads/develop".to_string(),
        before: "28e1879d029cb852e4844d9c718537df08844e03".to_string(),
        after: "bffeb74224043ba2feb48d137756c8a9331c449a".to_string(),
        compare_url: "http://localhost:3000/gitea/webhooks/compare/28e1879d029cb852e4844d9c718537df08844e03...bffeb74224043ba2feb48d137756c8a9331c449a".to_string(),
        commits: vec![
            Commit {
                id: "bffeb74224043ba2feb48d137756c8a9331c449a".to_string(),
                message: "Webhooks Yay!".to_string(),
                url: "http://localhost:3000/gitea/webhooks/commit/bffeb74224043ba2feb48d137756c8a9331c449a".to_string(),
                author: GitUser {
                    name: "Gitea".to_string(),
                    email: "someone@gitea.io".to_string(),
                    username: "gitea".to_string(),
                },
                committer: GitUser {
                    name: "Gitea".to_string(),
                    email: "someone@gitea.io".to_string(),
                    username: "gitea".to_string(),
                },
                timestamp: "2017-03-13T13:52:11-04:00".to_string(),
            },
        ],
        repository: Repository {
            id: 140,
            owner: GiteaUser {
                id: 1,
                login: "gitea".to_string(),
                full_name: "Gitea".to_string(),
                email: "someone@gitea.io".to_string(),
                avatar_url: "https://localhost:3000/avatars/1".to_string(),
                username: "gitea".to_string(),
            },
            name: "webhooks".to_string(),
            full_name: "gitea/webhooks".to_string(),
            description: "".to_string(),
            private: false,
            fork: false,
            html_url: "http://localhost:3000/gitea/webhooks".to_string(),
            ssh_url: "ssh://gitea@localhost:2222/gitea/webhooks.git".to_string(),
            clone_url: "http://localhost:3000/gitea/webhooks.git".to_string(),
            website: "".to_string(),
            stars_count: 0,
            forks_count: 1,
            watchers_count: 1,
            open_issues_count: 7,
            default_branch: "master".to_string(),
            created_at: "2017-02-26T04:29:06-05:00".to_string(),
            updated_at: "2017-03-13T13:51:58-04:00".to_string(),
        },
        pusher: GiteaUser {
            id: 1,
            login: "gitea".to_string(),
            full_name: "Gitea".to_string(),
            email: "someone@gitea.io".to_string(),
            avatar_url: "https://localhost:3000/avatars/1".to_string(),
            username: "gitea".to_string(),
        },
        sender: GiteaUser {
            id: 1,
            login: "gitea".to_string(),
            full_name: "Gitea".to_string(),
            email: "someone@gitea.io".to_string(),
            avatar_url: "https://localhost:3000/avatars/1".to_string(),
            username: "gitea".to_string(),
        },
    };
    assert_eq!(expected, parsed);
}
