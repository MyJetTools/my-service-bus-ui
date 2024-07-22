### MyServiceBus UI

Gives ability to monitor several MyServiceBus instances in one place.



### Settings file

please create ~/.my-sb-ui file with yaml content

```yaml
envs:
- id: evn-1
  sb_api_url: https://env-1-url/

- id: evn-2
  sb_api_url: https://env-2-url/
  cert_location: ~/cert.p12
  cert_password: password

- id: env-3
  sb_api_url: https://49.13.227.81
  host: domain.com

```