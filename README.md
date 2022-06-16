# zkbinfo

Consider to add to the /etc/hosts
```
<YOUR_SERVER>  zkbinfo
```
And then use next links:

### API section
##### Get zkbinfo statistic
http://zkbinfo:8080/api/statistic

##### Get saved killmail ids per date
```
http://zkbinfo:8080/api/killmail/ids/YYYY-MM-DD/
```
http://zkbinfo:8080/api/killmail/ids/2022-06-01/

##### Get character's report
```
http://zkbinfo:8080/api/character/report/{id}/
```
http://zkbinfo:8080/api/character/report/1099051589/

##### Get character's friends {/character/corporation/alliance}
```
http://zkbinfo:8080/api/character/friends/char/{id}/
http://zkbinfo:8080/api/character/friends/corp/{id}/
http://zkbinfo:8080/api/character/friends/alli/{id}/
```
http://zkbinfo:8080/api/character/friends/char/1099051589/

http://zkbinfo:8080/api/character/friends/corp/1099051589/

http://zkbinfo:8080/api/character/friends/alli/1099051589/

##### Get character's enemies {/character/corporation/alliance}
```
http://zkbinfo:8080/api/character/enemies/char/{id}/
http://zkbinfo:8080/api/character/enemies/corp/{id}/
http://zkbinfo:8080/api/character/enemies/alli/{id}/
```
http://zkbinfo:8080/api/character/enemies/char/1099051589/

http://zkbinfo:8080/api/character/enemies/corp/1099051589/

http://zkbinfo:8080/api/character/enemies/alli/1099051589/



### KILLMAIL section
#### Save killmail to the database
```
$ curl -X POST zkbinfo:8080/killmail/save -d @"zkbinfo/doc/killmail.json"
```
