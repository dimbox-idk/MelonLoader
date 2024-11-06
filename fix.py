import os

base_dir = r"C:\Users\portal2707070\Documents\GitHub\MelonLoader"

def get_relative_path(source, target):
    return os.path.relpath(target, start=source)

for root, dirs, files in os.walk(base_dir):
    for file in files:
        if file.endswith(".csproj"):
            file_path = os.path.join(root, file)

            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

            new_content = content.replace(
                '<PackageReference Include="Disarm" Version="2022.1.0-master.57" />',
                f'<ProjectReference Include="{get_relative_path(root, r"C:/Users/portal2707070/Documents/GitHub/MelonLoader/External/Disarm/Disarm/Disarm.csproj")}" />'
            )

            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
