from api_test import signup, login, create_group, join_group

# Test case setup
test_users = [
    {"username": "user_1", "password": "111", "access_token": ""},
    {"username": "user_2", "password": "222", "access_token": ""},
    {"username": "user_3", "password": "333", "access_token": ""},
    {"username": "user_4", "password": "444", "access_token": ""},
    {"username": "user_5", "password": "555", "access_token": ""},
    {"username": "user_6", "password": "666", "access_token": ""},
]

# Signup users
for user in test_users:
    signup_response = signup(user["username"], user["password"])
    # assert signup_response.status_code == 200, f"Signup failed for {user['username']}"

# Login users and store access tokens
for i, user in enumerate(test_users):
    login_response = login(user["username"], user["password"])
    assert login_response.status_code == 200, f"Login failed for {user['username']}"
    test_users[i]["access_token"] = login_response.json().get("access_token")

# Create groups
code_1 = create_group(
    test_users[0]["access_token"], "group_1").json().get("group_code")
code_2 = create_group(
    test_users[3]["access_token"], "group_2").json().get("group_code")
code_3 = create_group(
    test_users[0]["access_token"], "group_3").json().get("group_code")

# Join groups
assert join_group(test_users[1]["access_token"], code_1).status_code == 200
assert join_group(test_users[2]["access_token"], code_1).status_code == 200

assert join_group(test_users[4]["access_token"], code_2).status_code == 200
assert join_group(test_users[5]["access_token"], code_2).status_code == 200

assert join_group(test_users[1]["access_token"], code_3).status_code == 200
assert join_group(test_users[2]["access_token"], code_3).status_code == 200
assert join_group(test_users[3]["access_token"], code_3).status_code == 200
assert join_group(test_users[4]["access_token"], code_3).status_code == 200
assert join_group(test_users[5]["access_token"], code_3).status_code == 200

for user in test_users:
    print(
        f'{user["username"]}: \nwscat -c ws://localhost:8081/api/ws/connect -H "Authorization: Bearer {user["access_token"]}"\n')

# messages:
# {"group_id":1,"content":"hello world in group 1"}
# {"group_id":2,"content":"hello world in group 2"}
# {"group_id":3,"content":"hello world in group 3"}

# user_1:
# wscat -c ws://localhost:8081/api/ws/connect -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJleHAiOjE3MzQxNDkzOTl9.nA7oQnNhhodujhvXdfI-0p16xBLXBTKeRQlw6wKZZs0"

# user_2:
# wscat -c ws://localhost:8081/api/ws/connect -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoyLCJleHAiOjE3MzQxNDkzOTl9.FnpBMczpP3bE4-7Ul8pSH2jBQc2ZIOM3QrMpajD_MTM"

# user_3:
# wscat -c ws://localhost:8081/api/ws/connect -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjozLCJleHAiOjE3MzQxNDkzOTl9.FMyMMu7mDSC0i27vmq_kzTpqq7zYb3XU8CCSm1ClYo4"

# user_4:
# wscat -c ws://localhost:8081/api/ws/connect -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjo0LCJleHAiOjE3MzQxNDkzOTl9.-jjFhPo5eCoblBIcHpXuxIsT43CpcVUxed1MpcLg9p8"

# user_5:
# wscat -c ws://localhost:8081/api/ws/connect -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjo1LCJleHAiOjE3MzQxNDkzOTl9.Otf3gYzKd9K1-7Q61ejeHwwH3kD1s-6JaIshEMLYiQE"

# user_6:
# wscat -c ws://localhost:8081/api/ws/connect -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjo2LCJleHAiOjE3MzQxNDkzOTl9.UA7EkDNbOnAaURIz4DJYHdpV-Gr7AVy5eKWOTYQPDgs"
