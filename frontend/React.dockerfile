FROM node:14-alpine as build-stage

ENV REACT_APP_ID_API_URL=https://xam-xam.xyz:8000 
ENV REACT_APP_BIS_API_URL=https://xam-xam.xyz:8001

COPY xam-xam-react/ $HOME/xam-xam-react/

WORKDIR $HOME/xam-xam-react

RUN npm install && npm run build

# Final build state
FROM nginx:alpine
EXPOSE 443

RUN rm -rf /usr/share/nginx/html/*
RUN rm /etc/nginx/conf.d/default.conf

COPY --from=build-stage $HOME/xam-xam-react//build/ /usr/share/nginx/html
COPY ./nginx.conf /etc/nginx/nginx.conf

CMD ["nginx","-g","daemon off;"]