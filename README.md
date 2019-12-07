## Dependencies
1. actix-web
2. juniper
3. mongodb

## Run

```
cargo run

```

Then navigation to [http://localhost:8080/graphiql](http://localhost:8080/graphiql) 

# Query 

```graphql
{
  members{
    id
    name
  }
}

```

# Mutation

```graphql
mutation{
  createMember(
    newMember: {
      id: 6
      name: "Ratul"
    }
  ){
    id
    name
  }
}
```
