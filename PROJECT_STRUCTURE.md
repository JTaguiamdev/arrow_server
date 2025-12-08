# Arrow Server Project Structure

This document outlines the structure of the Arrow Server project.

```
C:\\Users\\owner\\PROJECTS GITHUB\\arrow_server\\
├───.dockerignore
├───.gitignore
├───android_backend_plan.md
├───Cargo.lock
├───Cargo.toml
├───diesel.toml
├───Dockerfile
├───LICENSE
├───Orderly-Android-API-Guide.md
├───README.md
├───.git\\
├───.github\\
│   ├───copilot-instructions.md
│   └───workflows\\
│       ├───arrow-server-v1-AutoDeployTrigger-2641f524-edcc-48ef-b656-c4eaf1376932.yml
│       └───rust.yml
├───ARROW_SERVER\\
│   ├───Arrow_server.bru
│   └───bruno.json
├───src\\
│   ├───lib.rs
│   ├───main.rs
│   ├───api\\
│   │   ├───config.rs
│   │   ├───errors.rs
│   │   ├───extractors.rs
│   │   ├───mod.rs
│   │   ├───server.rs
│   │   ├───controllers\\
│   │   │   ├───mod.rs
│   │   │   ├───order_controller.rs
│   │   │   ├───product_controller.rs
│   │   │   ├───role_controller.rs
│   │   │   ├───user_controller.rs
│   │   │   └───dto\\
│   │   │       ├───login_dto.rs
│   │   │       ├───mod.rs
│   │   │       ├───order_dto.rs
│   │   │       ├───product_dto.rs
│   │   │       ├───role_dto.rs
│   │   │       └───user_dto.rs
│   │   └───routes\\
│   │       ├───auth_routes.rs
│   │       ├───mod.rs
│   │       ├───order_routes.rs
│   │       ├───product_routes.rs
│   │       ├───role_routes.rs
│   │       └───user_routes.rs
│   ├───data\\
│   │   ├───database.rs
│   │   ├───mod.rs
│   │   ├───migrations\\
│   │   │   ├───.diesel_lock
│   │   │   ├───.keep
│   │   │   ├───2025-11-22-113224-0000_user\\
│   │   │   │   ├───down.sql
│   │   │   │   └───up.sql
│   │   │   ├───2025-11-24-082606-0000_roles\\
│   │   │   │   ├───down.sql
│   │   │   │   └───up.sql
│   │   │   ├───2025-11-27-104614-0000_products\\
│   │   │   │   ├───down.sql
│   │   │   │   └───up.sql
│   │   │   ├───2025-11-27-111653-0000_orders\\
│   │   │   │   ├───down.sql
│   │   │   │   └───up.sql
│   │   │   └───2025-11-27-115132-0000_order_products\\
│   │   │       ├───down.sql
│   │   │       └───up.sql
│   │   ├───models\\
│   │   │   ├───mod.rs
│   │   │   ├───order_product.rs
│   │   │   ├───order.rs
│   │   │   ├───product.rs
│   │   │   ├───schema.rs
│   │   │   ├───user_roles.rs
│   │   │   └───user.rs
│   │   └───repos\\
│   │       ├───mod.rs
│   │       ├───implementors\\
│   │       │   ├───mod.rs
│   │   │   │   ├───order_product_repo.rs
│   │   │   │   ├───order_repo.rs
│   │   │   │   ├───product_repo.rs
│   │   │   │   ├───user_repo.rs
│   │   │   │   └───user_role_repo.rs
│   │       │└───traits\\
│   │       │   ├───mod.rs
│   │       │   └───repository.rs
│   ├───security\\
│   │   ├───auth.rs
│   │   ├───errors.rs
│   │   ├───jwt.rs
│   │   └───mod.rs
│   ├───services\\
│   │   ├───azure_service.rs
│   │   ├───errors.rs
│   │   ├───mod.rs
│   │   ├───order_service.rs
│   │   ├───product_service.rs
│   │   └───role_service.rs
│   └───utils\\
│       ├───mappers.rs
│       └───mod.rs
└───tests\\
    ├───database_tests.rs
    ├───mod.rs
    ├───order_controller_tests.rs
    ├───order_product_repo_tests.rs
    ├───order_repo_tests.rs
    ├───order_service_tests.rs
    ├───product_controller_tests.rs
    ├───product_repo_tests.rs
    ├───product_service_tests.rs
    ├───role_controller_tests.rs
    ├───user_controller_tests.rs
    ├───user_repo_tests.rs
    └───user_role_repo_tests.rs
```