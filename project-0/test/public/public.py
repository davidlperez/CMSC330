import subprocess
import re

def get_version(pkg):
  ret = subprocess.run([pkg,'--version'],capture_output=True)
  version = ret.stdout.decode().strip() 
  matched = re.search(r'([0-9]+\.[0-9]+)',version)
  if matched:
    return matched.group(1),None
  return None,pkg + " not installed"

def get_opam_pkg(pkg):
  ret = subprocess.run(['opam','info',pkg],capture_output=True)
  version = ret.stdout.decode().strip() 
  matched = re.search(r'all-installed-versions\s+([0-9\.]+)',version)
  if matched:
    return matched.group(1),None
  return None,pkg + " not installed"

versions = {'ocaml':4.13,'opam':2.0,'python3':3.8,'pytest':7.1,'rustc':1.63}

def test_versions():
  pkgs = ['ocaml','opam','python3','pytest','rustc']
  version_re = r'([0-9]+)\.([0-9]+)'
  f = open('p0.report','a')
  for pkg in pkgs:
    ret,err = get_version(pkg)
    if err:
      assert False, err
    pkg_ver = re.search(version_re,ret)
    min_ver = re.search(version_re,str(versions[pkg]))
    if pkg_ver and min_ver:
      assert (int(pkg_ver.group(1)) >= int(min_ver.group(1)) and
             int(pkg_ver.group(2)) >= int(min_ver.group(2))),pkg+" version not high enough"
    else:
      assert False, "Could not parse Version"
    f.write(pkg+"="+str(ret)+'\n')

def test_opam():
  pkgs = ['ounit','dune','utop']
  f = open('p0.report','a')
  for pkg in pkgs:
    ret,err = get_opam_pkg(pkg)
    assert ret,err
    f.write(pkg+"="+str(ret)+'\n')
