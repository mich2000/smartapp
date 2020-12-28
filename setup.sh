git pull;

cp $1 ./docker-compose.yml;
cp $2 ./Jwt.toml;
cp ./Jwt.toml ./backend/Jwt.toml;

chmod +x ./build.sh;

./build.sh ./Jwt.toml && docker-compose down && docker-compose up -d;