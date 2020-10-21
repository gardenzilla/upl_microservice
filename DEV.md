## Public API

GET /upl/[UPL_ID] Get UPL by ID
GET /upl/[UPL_ID]/location Get UPL current location
POST /upl/new Create new UPL
GET /upl/place/[PLACE_ID] Get UPLs by place
GET /upl/[UPL_id]/history Get UPL history

## Private API

PUT /upl/[UPL_ID]/devide Devide UPL into two parts
PUT /upl/[UPL_ID]/merge Merge UPLs into a single one
PUT /upl/[UPL_ID]/move Move UPL
PUT /upl/[UPL_ID]/bestbefore Set best before date
PUT /upl/[UPL_ID]/culling Set culling: culling_id, description, culled_price
PUT /upl/[UPL_ID]/price Set retail price

rpc get_upl_by_id({
int32 upl_id = 1;
}) returns ({
upl upl_result = 1;
});

rpc get_upls_by_location({
int32 location_id = 1;
}) returns ({
repeated upl upls = 1;
});

rpc new_upl({
..
}) returns ({
upl upl_result = 1;
});

## Devide

```protobuf
rpc devide(DevideRequest) returns (DevideResponse);

message DevideRequest {
  int32 upl_id = 1; // UPL id to merge
  int32 target_upl_id = 2; // Target UPL id, scan it f sticker
  int32 target_upl_quantity = 3; // Target UPL quantity
}

message DevideResponse {
  upl source_upl = 1;
  upl target_upl = 2;
}
```

Merge
=====

```protobuf
rpc merge(MergeRequest) returns (MergeResponse);

message Upl {..}

message MergeRequest {
  int32 to_upl_id = 1;    // Merge the from UPL into this one
  int32 from_upl_id = 2;  // This upl is going to be merged
}

message MergeResponse {
  Upl upl_result = 1;
}
```

rpc move({
int32 location_from = 1;
int32 upl_id = 2;
int32 location_to = 3;
}) returns ({
upl upl_result = 1;
});

rpc best_before({
int32 upl_id = 1;
string best_before = 2;
}) returns ({
upl upl_result = 1;
});

rpc culling({
int32 upl_id = 1;
int32 culling_id = 2;
string description = 3;
float culled_net_retail_price = 4;
}) returns ({
upl upl_result = 1;
});

rpc set_price({
int32 upl_id = 1;
float net_retail_price = 2;
string vat = 3;
float gross_retail_price = 3;
}) returns ({
upl upl_result = 1;
});
