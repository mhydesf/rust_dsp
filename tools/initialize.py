import os

class ConfigurationError(Exception):
    pass

def main():
    
    if not os.path.exists("venv"):
        raise ConfigurationError("Please generate a DSP virtualenv in a venv/ directory within this project")
        
    if not os.path.exists("venv/lord/"):
        raise ConfigurationError("You have not created a virtual environment for the project named \"DSP\"")
    
    # Create all required directories
    root = "data"
    dirs = ["graph", "csv", "tmp"]
    
    # Create Data Folder
    if not os.path.exists(root):
        os.mkdir(root)
    else:
        pass      
    
    # Create Data Sub Folders
    for d in dirs:
        path = f"{root}/{d}"
        if not os.path.exists(path):
            os.mkdir(path)
    

if __name__ == '__main__':
    main()