git pull;

cp $1 ./docker-compose.yml;
cp $2 ./Jwt.toml;
cp ./Jwt.toml ./backend/Jwt.toml;

bash build.sh && docker-compose down && docker-compose up -d;