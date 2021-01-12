cp $1 ./docker-compose.yml;
cp $2 ./Jwt.toml;
cp ./Jwt.toml ./backend/Jwt.toml;

docker-compose down;

docker rmi xam-id:latest;
docker rmi xam-bis:latest;
docker rmi xam-web:latest;

bash ./build.sh ./Jwt.toml && docker-compose down && docker-compose up -d;