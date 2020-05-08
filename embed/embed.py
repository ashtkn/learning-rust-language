from ctypes import cdll

if __name__ == "__main__":
    lib = cdll.LoadLibrary('target/release/libembed.dylib')
    lib.process()
    print('done!')
