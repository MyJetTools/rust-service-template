# rust-service-template

reqiarements to base service:

1. read config by http or local file. Confit it is yaml structure. Env variable: SETTINGS_URL
  * if http url then get test via http
  * if not then read file from home directory

2. Need to add config for example 

3. need to listing http endpoint with monitor\isalive get method. Return json with app name and version. Port by default 8080, but we can specify in env var HTTP_PORT

4. need to have log system. Logs shold writes to console and writes to ELK
 * levels: Information, Warning, Error. Debug - nice to have
 * in example settings file provide settings for logs (ELK)
 
5. Add busines logic service 
 * interface: with SayHello: Reques= { Name } and Responce = { Name, Message }
 * implementation: `_count++; Response.Message = "Hello " + Name + ". Counter=" + _count; Response.Name=Name;`. Where count. - it is how many time call methods. 
 
6. need to have demo GRPC controller and listen on 80 port http2 enpoint
* Hello World service with one method SayHallo. Reques= { Name } and Responce = { Name, Message }
* proto files shold return via http at :8080/grpc
* has dependency to 6.interface and just call Say hello

7. MAke a client library for the service

 
 



 


