# sharky
make and store ssh sessions. 



## Vision 
 ```$ sharky```

```
1: Linode
2. GCP
3. AWS
....
``` 
```sharky> list```

it should go ahead and grab the first item, and strat an ssh session.


# Valid Commands
```
list
-join <NUMBER OF INDEX>
-create --username <user name> --ip <IP address> --password <password>
```


# Refrences 
https://github.com/manuels/libssh.rs/blob/master/example.rs
https://docs.rs/ssh/latest/ssh/

## Sample Json
```json
[
  {
    "name": "linode",
    "ip": "12376",
    "username": "root",
    "password": "shdasd"
  },
  {
    "name": "AWS",
    "ip": "172.3.44.3",
    "username": "root",
    "password": "shdasd"
  },
  {
    "name": "GCP",
    "ip": "173.2.42.0",
    "username": "root",
    "password": "shdasd"
  }
]
```

