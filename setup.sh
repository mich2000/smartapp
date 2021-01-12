cp $1 ./docker-compose.yml;
cp $2 ./Jwt.toml;
cp ./Jwt.toml ./backend/Jwt.toml;

docker-compose down;

docker rmi xam-id:latest;
docker rmi xam-bis:latest;
docker rmi xam-web:latest;

docker rmi xam-id:staging;
docker rmi xam-bis:staging;
docker rmi xam-web:staging;

bash ./build.sh ./Jwt.toml && docker-compose down && docker-compose up -d;