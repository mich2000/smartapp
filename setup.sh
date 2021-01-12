cp $1 ./docker-compose.yml;
cp $2 ./Jwt.toml;
cp ./Jwt.toml ./backend/Jwt.toml;

docker-compose down;

docker system prune -fa;

bash ./build.sh ./Jwt.toml && docker-compose down && docker-compose up -d;