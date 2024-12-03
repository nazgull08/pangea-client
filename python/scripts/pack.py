import os
import shutil
import subprocess


def clean():
    """Clean the dist/ and target/package directories"""

    if os.path.exists("dist"):
        shutil.rmtree("dist")

    if os.path.exists("target/package"):
        shutil.rmtree("target/package")


def edit_readme():
    """Edit README.md to remove all lines after line 30"""

    readme_path = "README.md"

    # Edit the README.md
    with open(readme_path, "r") as f:
        lines = f.readlines()

    with open(readme_path, "w") as f:
        f.writelines(lines[:28])


def restore_readme():
    """Restore README.md to its original state using git"""

    subprocess.run(["git", "restore", "README.md"], check=True)


def pack():
    """Build the package using Poetry and move it to target/package"""

    subprocess.run(["poetry", "build"], check=True)
    os.makedirs("target/package", exist_ok=True)

    # move files from dist/ to target/package/
    for filename in os.listdir("dist"):
        shutil.move(os.path.join("dist", filename), "target/package/")
    print("Package moved to target/package/")

    if os.path.exists("dist"):
        shutil.rmtree("dist")


def publish():
    """Publish the package to PyPI using Poetry"""

    print("Publishing the package to PyPI...")
    subprocess.run(["poetry", "publish", "--dist-dir", "target/package/"], check=True)
    print("Package published to PyPI!")


if __name__ == "__main__":
    clean()
    edit_readme()
    try:
        pack()
        publish()
    finally:
        restore_readme()
