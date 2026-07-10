# Mongodb

[toc]

# General

- text indexes: https://www.mongodb.com/docs/manual/core/index-text/#std-label-index-feature-text

search / filter / find data

```text
db.bios.find( { _id: 5 } )
db.bios.find( { "name.last": "Hopper" } )
```

show / list all dbs

```text
show dbs
```

enter / use a db

```text
use <db_name>
```

show / list all collections

```text
show collections
db.getCollectionNames()
```

---

**Check where db is stored in disk (directory for database)**
https://stackoverflow.com/questions/7247474/how-can-i-tell-where-mongodb-is-storing-data-its-not-in-the-default-data-db

```text
db.serverCmdLineOpts()
```

# Indexes

```text
db.collection.getIndexes()
db.pets.dropIndex( "catIdx" )
```

# Mongosh Mongo Shell

[How to execute mongo commands through shell scripts?](https://stackoverflow.com/questions/4837673/how-to-execute-mongo-commands-through-shell-scripts)

```text
mongosh --eval "printjson(db.serverStatus())"
mongosh --eval 'db.mycollection.update({"name":"foo"},{$set:{"this":"that"}});' myDbName
mongosh yourFile.js
```
