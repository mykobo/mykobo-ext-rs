# geolocation
Get geolocation information of an IP. Its simple.
```sh
geolocation = "0.2.1"
```
Add this line to your Cargo.toml.

### Example
Using geolocation is really quite easy and simple:
```sh
use geolocation;
fn main() {
    let ip = "<Put your IP address Here>";
    let info = geolocation::find(ip).unwrap();
 
    println!("{:?}", info.city);
}

```

This and more examples are found in the examples directory.

### Query Limits
You can send 45 requests per minute.

### Fields
The API can get these fields about IP addresses.
```sh
ip
latitude
longitude
city
region
country
timezone
location
```
