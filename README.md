# zkbinfo

Consider to add to the /etc/hosts
```
<YOUR_SERVER>  zkbinfo
```
And then use next links:

### GUI Section

- Perform analysis against local or any list of character names with /r/n delimiter

   http://185.87.51.139:8088/gui/who/

- Get information abount character

   http://185.87.51.139:8088/gui/character/Seb Odessa/ (http://185.87.51.139:8088/gui/character/id/2114350216/)

- Get information abount corporation
http://185.87.51.139:8088/gui/corporation/SO%20Corporation/ (http://185.87.51.139:8088/gui/corporation/id/98573194/)

- Get information abount alliance

http://185.87.51.139:8088/gui/alliance/Train%20Wreck./ (http://185.87.51.139:8088/gui/alliance/id/99011258/)







### API section
##### Get zkbinfo statistic
http://185.87.51.139:8080/api/statistic

##### Get saved killmail ids per date
```
http://185.87.51.139:8080/api/killmail/ids/YYYY-MM-DD/
```
http://185.87.51.139:8080/api/killmail/ids/2022-06-01/


### Get activity

```
http://185.87.51.139:8080/api/<character|corporation|alliance>/activity/{id}/

```

e.g.:

http://185.87.51.139:8080/api/character/activity/1099051589/

### Get activity hourly
```
http://185.87.51.139:8080/api/<character|corporation|alliance>/activity/hourly/{id}/
```

e.g.:

http://185.87.51.139:8080/api/character/activity/hourly/1099051589/

### Get friendly character|corporation|alliance
```
http://185.87.51.139:8080/api/<character|corporation|alliance>/friends/char/{id}/
http://185.87.51.139:8080/api/<character|corporation|alliance>/friends/corp/{id}/
http://185.87.51.139:8080/api/<character|corporation|alliance>/friends/alli/{id}/
```

e.g.:

http://185.87.51.139:8080/api/character/friends/char/1099051589/

http://185.87.51.139:8080/api/character/friends/corp/1099051589/

http://185.87.51.139:8080/api/character/friends/alli/1099051589/

http://185.87.51.139:8080/api/corporation/friends/char/98575144/

http://185.87.51.139:8080/api/corporation/friends/corp/98575144/

http://185.87.51.139:8080/api/corporation/friends/alli/98575144/

http://185.87.51.139:8080/api/alliance/friends/char/99010079/

http://185.87.51.139:8080/api/alliance/friends/corp/99010079/

http://185.87.51.139:8080/api/alliance/friends/alli/99010079/


### Get enemies character|corporation|alliance
```
http://185.87.51.139:8080/api/<character|corporation|alliance>/enemies/char/{id}/
http://185.87.51.139:8080/api/<character|corporation|alliance>/enemies/corp/{id}/
http://185.87.51.139:8080/api/<character|corporation|alliance>/enemies/alli/{id}/
```

e.g.:

http://185.87.51.139:8080/api/character/enemies/char/1099051589/

http://185.87.51.139:8080/api/character/enemies/corp/1099051589/

http://185.87.51.139:8080/api/character/enemies/alli/1099051589/

http://185.87.51.139:8080/api/corporation/enemies/char/98575144/

http://185.87.51.139:8080/api/corporation/enemies/corp/98575144/

http://185.87.51.139:8080/api/corporation/enemies/alli/98575144/

http://185.87.51.139:8080/api/alliance/enemies/char/99010079/

http://185.87.51.139:8080/api/alliance/enemies/corp/99010079/

http://185.87.51.139:8080/api/alliance/enemies/alli/99010079/






### KILLMAIL section
#### Save killmail to the database
```
$ curl -X POST 185.87.51.139:8080/killmail/save -d @"zkbinfo/doc/killmail.json"
```
