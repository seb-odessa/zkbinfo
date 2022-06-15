# zkbinfo

The base path: http://localhost:8080/

### API section

#### Get current statistic
http://localhost:8080/api/stat
```
$ curl http://localhost:8080/api/stat
```


#### Get saved killmail ids per date
http://localhost:8080/api/killmail/saved/YYYY-MM-DD/

e.g.:
http://localhost:8080/api/killmail/saved/2022-06-01/
http://localhost:8080/api/killmail/saved/2022-06-02/
http://localhost:8080/api/killmail/saved/2022-06-03/
```
$ curl http://localhost:8080/api/killmail/saved/2022-06-02/
```

### KILLMAIL section
#### Saved killmail to the database
```
$ curl -X POST localhost:8080/killmail/save -d @"zkbinfo/doc/killmail.json"
```
