# zkbinfo

Consider to add to the /etc/hosts

```<YOUR_SERVER>  zkbinfo```
And then use nex links:

### API section

#### Get current statistic
http://zkbinfo:8080/api/stat
```
$ curl http://zkbinfo:8080/api/stat
```


#### Get saved killmail ids per date
http://zkbinfo:8080/api/killmail/saved/YYYY-MM-DD/

e.g.:

http://zkbinfo:8080/api/killmail/saved/2022-06-01/

http://zkbinfo:8080/api/killmail/saved/2022-06-02/

http://zkbinfo:8080/api/killmail/saved/2022-06-03/
```
$ curl http://zkbinfo:8080/api/killmail/saved/2022-06-02/
```

#### Get character's report
http://zkbinfo:8080/api/character/report/{id}/

e.g.:

http://zkbinfo:8080/api/character/report/1099051589/


### KILLMAIL section
#### Saved killmail to the database
```
$ curl -X POST zkbinfo:8080/killmail/save -d @"zkbinfo/doc/killmail.json"
```
