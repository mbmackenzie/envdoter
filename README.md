
# EnvDoter

## Introduction
EnvDoter is a command-line tool designed to simplify the management of environment variables and `.env` files across different projects. It provides functionalities such as creating new `.env` files, generating files from templates, securely adding and including variables, and more. The tool also features the ability to create Kubernetes secrets files from `.env` files, making it an invaluable asset in modern development environments.

## Features
- Create new blank `.env` files.
- Generate `.env` files using customizable templates.
- Securely add and manage environment variables.
- Easily include variables in your `.env` file.
- Generate `.env-sample` files.
- Create Kubernetes secrets files from `.env` files.
- List, remove, and edit environment variables.

## Installation
[Instructions for installing EnvDoter]

## Usage
[Instructions for using EnvDoter]

## EnvDoter Commands Checklist

| Complete | Feature ID    | Feature Description                                         | Priority |
| -------- | ------------- | ----------------------------------------------------------- | -------- |
| X        | Init          | Create a new blank .env file                                | High     |
|          | Generate      | Create a new .env file using a template                     | High     |
| X        | Add           | Add a variable to a database                                | High     |
|          | Include       | Include a variable in the .env file                         | High     |
|          | Create-Sample | Generate a .env-sample file from the current .env file      | High     |
|          | Secrets       | Create Kubernetes secrets file from the .env file           | High     |
|          | List (ls)     | Display a list of stored environment variables or .env file | Medium   |
|          | Remove (rm)   | Remove a variable from the .env file or database            | Medium   |
|          | Edit          | Edit the .env file or a specific variable                   | Medium   |
|          | Export        | Export the .env file to different formats                   | Medium   |
|          | Sync          | Synchronize .env files across different projects            | Medium   |
|          | Diff          | Compare two .env files and show differences                 | Low      |
|          | Backup        | Create a backup of the current .env file                    | Low      |
|          | Restore       | Restore an .env file from a backup                          | Low      |
|          | Validate      | Check the .env file for errors or inconsistencies           | Low      |
|          | Merge         | Combine variables from multiple .env files                  | Low      |
|          | Rename        | Rename an environment variable in the .env file             | Low      |
