cp $1 ./docker-compose.yml;
cp $2 ./Jwt.toml;
cp ./Jwt.toml ./backend/Jwt.toml;

docker-compose down;

docker rmi xam-id;
docker rmi xam-bis;

bash ./build.sh ./Jwt.toml && docker-compose down && docker-compose up -d;