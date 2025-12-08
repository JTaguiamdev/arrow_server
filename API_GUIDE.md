
# Arrow Server API Guide

This document provides a guide on how to communicate with the Arrow Server API. The following endpoints are available:

---

## Authentication

### Login

- **Description:** Authenticate a user and receive a JWT token.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/auth/login`
- **Body:**
  ```json
  {
    "username": "adminuser",
    "password": "adminpasword"
  }
  ```

### Register

- **Description:** Register a new user.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/auth/register`
- **Body:**
  ```json
  {
    "username": "adminuser",
    "password": "adminpassword"
  }
  ```

### Refresh

- **Description:** Refresh a JWT token.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/auth/refresh`

---

## Orders

### Create Order

- **Description:** Create a new order.
- **Method:** `POST`
- **URL:** `http://127.0.0.1:3000/api/v1/orders`
- **Body:**
  ```json
  {
    "products": [
      {
        "product_id": 1,
        "quantity": 2
      },
      {
        "product_id": 2,
        "quantity": 1
      }
    ]
  }
  ```

### Get All Orders

- **Description:** Retrieve all orders.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/orders`

### Get Order By ID

- **Description:** Retrieve an order by its ID.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/orders/1`

### Get User Orders By Username

- **Description:** Retrieve all orders for a specific user.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/orders/user/testuser`

### Update Order

- **Description:** Update an existing order.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/orders/1`
- **Body:**
  ```json
  {
    "status": "cancelled"
  }
  ```

---

## Products

### Create Product

- **Description:** Create a new product.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/products`
- **Body:**
  ```json
  {
    "name": "Burger",
    "description": "Delicious beef burger with cheese",
    "price": "9.99",
    "product_image_uri": "/images/burger.jpg"
  }
  ```

### Get All Products

- **Description:** Retrieve all products.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/products`

### Get Product By ID

- **Description:** Retrieve a product by its ID.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/products/1`

### Update Product

- **Description:** Update an existing product.
- **Method:** `PUT`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/products/1`
- **Body:**
  ```json
  {
    "name": "Double Burger",
    "description": "Double beef burger with extra cheese",
    "price": "14.99",
    "product_image_uri": "/images/double-burger.jpg"
  }
  ```

### Delete Product

- **Description:** Delete a product by its ID.
- **Method:** `DELETE`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/products/1`

---

## Roles

### Create Role

- **Description:** Create a new role.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/roles/create`
- **Body:**
  ```json
  {
    "username": "testuser",
    "name": "admin",
    "description": "Administrator role with full access"
  }
  ```

### Get All Roles

- **Description:** Retrieve all roles.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/roles`

### Update Role

- **Description:** Update an existing role.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.ioapi/v1/roles/update/2`
- **Body:**
  ```json
  {
    "name": "customer",
    "description": "customer on the restaurant"
  }
  ```

### Delete Role

- **Description:** Delete a role by its ID.
- **Method:** `DELETE`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/roles/1`

### Set Permission

- **Description:** Set a permission for a role.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/roles/2/set_permission`
- **Body:**
  ```json
  {
    "permission": "read"
  }
  ```

### Remove Permission

- **Description:** Remove a permission from a role.
- **Method:** `PATCH`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/roles/1/delete_permission`
- **Body:**
  ```json
  {
    "permission": "read"
  }
  ```

### Assign Role To User

- **Description:** Assign a role to a user.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/roles/assign`
- **Body:**
  ```json
  {
    "username": "testuser",
    "role_name": "admin"
  }
  ```

---

## Users

### Get All Users

- **Description:** Retrieve all users.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/users`

### Get User By ID

- **Description:** Retrieve a user by their ID.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/users/2`

### Search User By Name

- **Description:** Search for a user by their username.
- **Method:** `GET`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/users/search?username=newuser`
- **Query Params:**
  - `username`: newuser

### Edit User

- **Description:** Edit a user's information.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/users/1`
- **Body:**
  ```json
  {
    "username": "updateduser",
    "password": "updatedpassword"
  }
  ```

### Delete User

- **Description:** Delete a user by their ID.
- **Method:** `DELETE`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/users/1`

### Add User

- **Description:** Add a new user.
- **Method:** `POST`
- **URL:** `https://arrow-server-v1.nicerock-8289607a.southeastasia.azurecontainerapps.io/api/v1/users/create`
- **Body:**
  ```json
  {
    "username": "addeduser",
    "password": "addedpassword"
  }
  ```
