### MyServiceBus UI

Gives ability to monitor several MyServiceBus instances in one place.



### Settings file

please create ~/.my-sb-ui file with yaml content

```yaml
envs:
- id: evn-1
  sb_api_url: https://env-1-url/

- id: evn-2
  sb_api_url: "ssh:test@10.0.0.0:22->http://localhost:5001"

- id: env-3
  sb_api_url: https://49.13.227.81
  host: domain.com

ssh_private_keys:
  ssh:test@10.0.0.0:22:
    cert_path: ~/.ssh/id_rsa
    cert_pass_phrase: pass 

```