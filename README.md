# bsp_dump
tool for dumping .bsp files of (hopefully, in the future) all source engine games

# usage
```
bsp_dump <bsp file>
```
example:
```
bsp_dump folder123/file.bsp
```
will output ```folder123/file-bsp_dump.txt``` (and a ```file-pakfile_dump.zip``` if bsp contains a pakfile)

# supported lumps

✅ - supported

❌ - not supported

🟨 - partial/possibly incorrect parsing

## Quake 1 (bsp version 29) / GoldSrc (bsp version 30)
|    lump name |   supported? |
|--------------|--------------|
|     entities |            ✅ |
|       planes |            ✅ |
|     textures |            ✅ |
|     vertices |            ✅ |
|   visibility |            ❌ |
|        nodes |            ✅ |
|      texinfo |            ✅ |
|        faces |            ✅ |
|     lighting | 🟨 (only goldsrc) |
|    clipnodes |            ✅ |
|       leaves |            ✅ |
| marksurfaces |            ✅ |
|        edges |            ✅ |
|    surfedges |            ✅ |
|       models |            ✅ |

## Portal/HL2 (vbsp)
|                   lump name | supported? |
|-----------------------------|------------|
|                    entities |          ✅ |
|                      planes |          ✅ |
|                     texdata |          ✅ |
|                    vertices |          ✅ |
|                  visibility |          🟨 |
|                       nodes |          ✅ |
|                     texinfo |          ✅ |
|                       faces |          ✅ |
|                    lighting |          ✅ |
|                   occlusion |          ✅ |
|                      leaves |          ✅ |
|                     faceids |          ✅ |
|                       edges |          ✅ |
|                   surfedges |          ✅ |
|                      models |          ✅ |
|                 worldlights |          ❌ |
|                   leaffaces |          ❌ |
|                 leafbrushes |          ❌ |
|                     brushes |          ✅ |
|                  brushsides |          ✅ |
|                       areas |          ✅ |
|                 areaportals |          ✅ |
|            unused22/portals |          ❌ |
|           unused23/clusters |          ❌ |
|        unused24/portalverts |          ❌ |
|     unused25/clusterportals |          ❌ |
|                    dispinfo |          ✅ |
|               originalfaces |          ✅ |
|                     phydisp |          ✅ |
|                 physcollide |          ✅ |
|                 vertnormals |          ✅ |
|           vertnormalindices |          ✅ |
|          displightmapalphas |          ❌ |
|                   dispverts |          ✅ |
| displightmapsamplepositions |          ✅ |
|          gamelump (headers) |          ✅ |
|               leafwaterdata |          ❌ |
|                  primitives |          ✅ |
|                   primverts |          ✅ |
|                 primindices |          ✅ |
|                     pakfile |          ✅ |
|             clipportalverts |          ✅ |
|                    cubemaps |          ✅ |
|           texdatastringdata |          ✅ |
|          texdatastringtable |          ✅ |
|                    overlays |          ✅ |
|          leafmindisttowater |         🟨 |
|        facemacrotextureinfo |         🟨 |
|                    disptris |          ✅ |

everything after that is not supported yet