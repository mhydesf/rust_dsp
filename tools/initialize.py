import os

def main():
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