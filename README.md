# zkbinfo

Consider to add to the /etc/hosts
```
<YOUR_SERVER>  zkbinfo
```
And then use next links:

### API section
##### Get zkbinfo statistic
http://zkbinfo:8080/api/stat

##### Get saved killmail ids per date
http://zkbinfo:8080/api/killmail/saved/YYYY-MM-DD/
http://zkbinfo:8080/api/killmail/saved/2022-06-01/

##### Get character's report
http://zkbinfo:8080/api/character/report/{id}/
http://zkbinfo:8080/api/character/report/1099051589/

##### Get character's friends
http://zkbinfo:8080/api/character/friends/{id}/
http://zkbinfo:8080/api/character/friends/1099051589/

##### Get character's enemies
http://zkbinfo:8080/api/character/enemies/{id}/
http://zkbinfo:8080/api/character/enemies/1099051589/



### KILLMAIL section
#### Save killmail to the database
```
$ curl -X POST zkbinfo:8080/killmail/save -d @"zkbinfo/doc/killmail.json"
```
