FROM node:14-alpine as build-stage

ENV REACT_APP_ID_API_URL=https://xam-xam.xyz:8000 
ENV REACT_APP_BIS_API_URL=https://xam-xam.xyz:8001

WORKDIR /app
COPY ./xam-xam-react/package*.json /app/

RUN npm install
COPY ./xam-xam-react/ /app/

RUN npm run build

# Final build state
FROM nginx:alpine
EXPOSE 443

RUN rm -rf /usr/share/nginx/html/*
RUN rm /etc/nginx/conf.d/default.conf

COPY --from=build-stage /app/build/ /usr/share/nginx/html
COPY ./nginx.conf /etc/nginx/nginx.conf

CMD ["nginx","-g","daemon off;"]