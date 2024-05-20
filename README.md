# Commands
## Debug inside container
### CURL
- apt-get update && apt-get install -y curl
- curl http://localhost:8000/

### PROCESSES
- apt-get update && apt-get install -y procps
- ps aux

### Build Docker image
docker build -t my-rust-app -f .\.dockerfile .

### Run Docker container
- docker run -p 8000:8000 --rm my-rust-app
- docker run -it my-rust-app /bin/bash

### Open interactive shell inside existing Docker container
docker exec -it container_IP /bin/bash

