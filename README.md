# logfmt

## Usage:
Extracts information from
```json
{
  "message": "heyho SomeClass1(id = abcdefg) and also SomeClass2(id=asda987das9d, name=Name(value=myName)) and some text afterwards",
  "logger_name": "de.company.package.logging.MyService",
  "level": "DEBUG",
  "traceId": "as8dzoa98s6zd",
}
```

and outputs:

```bash
$ cat tests/random.json | target/debug/logformat
{
  "level": "DEBUG",
  "logger_name": "de.company.package.logging.MyService",
  "message": "heyho {{ Log Entity 1 }} and also {{ Log Entity 2 }} and some text afterwards",
  "traceId": "as8dzoa98s6zd"
}
Log Entity 1 = {
  "id": "abcdefg"
}
Log Entity 2 = {
  "id": "asda987das9d", 
  "name": {
    "value": "myName"
  }
}
```