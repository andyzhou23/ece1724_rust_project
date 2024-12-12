for i in {1..20}; do
    curl -w "\n" "http://localhost:8080/signup" --json "{\"username\":\"user_$i\", \"password\":\"123456\"}"
done
for i in {1..20}; do
    curl -w "\n" "http://localhost:8080/login" --json "{\"username\":\"user_$i\", \"password\":\"123456\"}"
done