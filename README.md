# Welcome to Loco :train:

Loco is a web and API framework running on Rust.

This is the **Rest API starter** which includes a `User` model and authentication based on JWT.

## Quick Start

You need:

- A local postgres instance
- A local Redis instance

Check out your development [configuration](config/development.yaml).

> To configure a database , please run a local postgres database with <code>loco:loco</code> and a db named <code>[app name]\_development.</code>:
> <code>docker run -d -p 5432:5432 -e POSTGRES_USER=loco -e POSTGRES_DB=[app name]\_development -e POSTGRES_PASSWORD="loco" postgres:15.3-alpine</code>

Now start your app:

```
$ cargo loco start
Finished dev [unoptimized + debuginfo] target(s) in 21.63s
    Running `target/debug/myapp start`

    :
    :
    :

controller/app_routes.rs:203: [Middleware] Adding log trace id

                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀

started on port 3000
```

## Getting help

Check out [a quick tour](https://loco.rs/docs/getting-started/tour/) or [the complete guide](https://loco.rs/docs/getting-started/guide/).

## Start locally

### Docker command

```
$ docker run -d -p 5432:5432 -e POSTGRES_USER=loco -e POSTGRES_DB=oblivio_loco_be_development -e POSTGRES_PASSWORD="loco" postgres:15.3-alpine
$ docker run -d -p 5433:5432 -e POSTGRES_USER=loco -e POSTGRES_DB=oblivio_loco_be_test -e POSTGRES_PASSWORD="loco" postgres:15.3-alpine
$ docker run -p 6379:6379 -d redis redis-server
```

### Entity creation

#### REGISTRAZIONE

```
curl --location '127.0.0.1:3000/api/auth/register' \
     --header 'Content-Type: application/json' \
     --data-raw '{
         "first_name": "Gianluca",
         "last_name": "Moretti",
         "email": "gianmoretti@gmail.com",
         "password": "12341234"
     }'
```

#### LOGIN

```
curl --location '127.0.0.1:3000/api/auth/login' \
 --header 'Content-Type: application/json' \
 --data-raw '{
"email": "gianmoretti@loco.rs",
"password": "12341234"
}'
```

RESPONSE

```
{"token":"eyJ0e...,"pid":"03012829-48cf-43ad-831e-cdba5b2acd17","first_name":"Gianluca","last_name":"Moretti","is_verified":false}%
```

#### CURRENT

```
curl --location --request GET '127.0.0.1:3000/api/user/current' \
     --header 'Content-Type: application/json' \
     --header 'Authorization: Bearer ...'
```

#### ASSET

No Auth

```
curl --location --request POST '127.0.0.1:3000/api/assets' \
     --header 'Content-Type: application/json' \
      --data-raw '{
         "name": "Asset 1",
         "description": "Asset 1 description",
         "category": "Category 1"
     }'
```

Auth

```
curl --location --request POST '127.0.0.1:3000/api/assets' \
     --header 'Content-Type: application/json' \
      --header 'Authorization: Bearer ...' \
      --data-raw '{
         "name": "Asset 2",
         "description": "Asset 2 description",
         "category": "Category 2"
     }'
```

#### DESIGNATED

```
curl --location --request POST '127.0.0.1:3000/api/designateds' \
     --header 'Content-Type: application/json' \
      --header 'Authorization: Bearer ...' \
      --data-raw '{
          "email": "gino@example.com",
          "first_name": "Gino",
          "last_name": "Doe",
          "birth_date": "1978-04-02",
          "birth_place": "New York",
          "residence": "California",
          "phone_number": "+123456789",
          "fiscal_code": "ABCD1234E",
          "color": "blue",
               "image_url": "https://example.com/image.jpg"
     }'
```

#### ASSET DESIGNATED

```
curl --location --request POST '127.0.0.1:3000/api/asset_designateds' \
     --header 'Content-Type: application/json' \
      --header 'Authorization: Bearer ...' \
      --data-raw '{
          "asset_id": 1,
          "designated_id": 1
     }'
```

#### GET CURENT USER

```
curl --location --request GET '127.0.0.1:3000/api/user/current' \
     --header 'Content-Type: application/json' \
     --header 'Authorization: Bearer ...'
```

### Step di generazione delle entità (dal controller fino alle migrazioni)

```
$ cargo loco generate scaffold designated email:string! first_name:string! last_name:string! birth_date:ts birth_place:string residence:string phone_number:string fiscal_code:string color:string image_url:string

$ cargo loco generate scaffold asset name:string! description:text! category:string!

$ cargo loco generate scaffold asset_document mime_type:string! name:string! filename:string! url:string asset:references

$ cargo loco generate scaffold asset_designated asset:references designated:references
```

### Docker

```
docker build -f ./dockerfile -t demo-rust .

docker network create demo-rust-network

docker run -d -p 5432:5432 --network=demo-rust-network -e POSTGRES_USER=loco -e POSTGRES_DB=oblivio_loco_be_development -e POSTGRES_PASSWORD="loco" postgres:15.3-alpine

docker run -p 6379:6379 -d --network=demo-rust-network redis redis-server

docker run -d -p 3000:3000 --network=demo-rust-network demo-rust start

-- per estrarre gli ip di redis e postgres da infilare nelle configurazioni di deploy
docker inspect <container id> | grep -i "IPaDDreSS"
```

# Deploy on shuttle

```
$ cargo shuttle run (per local development)
$ cargo check
$ cargo build
$ cargo shuttle project start
$ cargo shuttle project status
$ cargo shuttle deploy --allow-dirty --no-test
```
