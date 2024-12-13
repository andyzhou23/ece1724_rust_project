import requests

BASE_URL = "http://localhost:8081"


def signup(username, password):
    response = requests.post(
        f"{BASE_URL}/signup", json={"username": username, "password": password})
    return response


def login(username, password):
    response = requests.post(
        f"{BASE_URL}/login", json={"username": username, "password": password})
    return response


def create_group(access_token, group_name):
    headers = {"Authorization": f"Bearer {access_token}"}
    response = requests.post(
        f"{BASE_URL}/api/group/create", json={"name": group_name}, headers=headers)
    return response


def list_groups(access_token):
    headers = {"Authorization": f"Bearer {access_token}"}
    response = requests.get(
        f"{BASE_URL}/api/group/list", headers=headers)
    return response


def join_group(access_token, group_code):
    headers = {"Authorization": f"Bearer {access_token}"}
    response = requests.post(
        f"{BASE_URL}/api/group/join", json={"group_code": group_code}, headers=headers)
    return response


def leave_group(access_token, group_id):
    headers = {"Authorization": f"Bearer {access_token}"}
    response = requests.post(
        f"{BASE_URL}/api/group/leave", json={"group_id": group_id}, headers=headers)
    return response


def generate_password(username):
    return f"{username*3}"


def main():
    users = [f"user_{i}" for i in range(8)]
    access_tokens = []
    group_codes = []
    # Signup users
    for username in users:
        signup_response = signup(username, generate_password(username))
        print(f"{username}: /signup -> {signup_response.json()}")
    print()

    # Login users and store access tokens
    for username in users:
        login_response = login(username, generate_password(username))
        access_token = login_response.json().get("access_token")
        access_tokens.append(access_token)
        print(f"{username}: /login -> {login_response.json()}")
    print()

    # Group actions
    for i in range(3):
        user_index = [0, 2, 3][i]
        create_group_response = create_group(
            access_tokens[user_index], f"group_{i}")
        print(
            f"user_{user_index}: /group/create -> {create_group_response.json()}")
        group_codes.append(create_group_response.json().get("group_code"))
        for j, access_token in enumerate(access_tokens):
            if j % (i+1) == 0:
                join_group_response = join_group(
                    access_token, group_codes[i])
                print(
                    f"user_{j}: /group/join -> {join_group_response.json()}")
        print()
    print()
    for i, access_token in enumerate([access_tokens[0], access_tokens[2], access_tokens[3]]):
        group_list_response = list_groups(access_token)
        groups = group_list_response.json()
        print(f"user_{i+1}: /group/list -> ")
        for j, group in enumerate(groups):
            print(
                f"{j+1}/{len(groups)}: {group['name']} | id: {group['id']} | code: {group['code']}\t", end="")
            for member in group['members']:
                print(
                    f"{member['name']}", end=", ")
            print()
        print()


if __name__ == "__main__":
    main()
