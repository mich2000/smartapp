git pull;

cp $1 ./docker-compose.yml;
cp $2 ./Jwt.toml;
cp ./Jwt.toml ./backend/xam-xam-bis-web/Jwt.toml;
cp ./Jwt.toml ./backend/xam-xam-id-web/Jwt.toml;

chmod +x ./build.sh;

./build.sh && docker-compose down && docker-compose up -d;