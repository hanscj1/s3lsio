// Copyright 2016 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_imports)]
#![allow(unused_variables)]

use clap::ArgMatches;
use aws_sdk_rust::aws::errors::s3::S3Error;
use aws_sdk_rust::aws::common::credentials::AwsCredentialsProvider;
use aws_sdk_rust::aws::common::request::DispatchSignedRequest;
use aws_sdk_rust::aws::s3::bucket::*;

use term;
use Client;

pub fn commands<P, D>(matches: &ArgMatches,
                      client: &mut Client<P,D>)
                      -> Result<(), S3Error>
                      where P: AwsCredentialsProvider,
                            D: DispatchSignedRequest {
    let bucket = matches.value_of("name").unwrap_or("");

    match matches.subcommand() {
      ("acl", Some(sub_matches)) => {
        let error = format!("not implemented yet!");
        println_color_quiet!(client.is_quiet, term::color::RED, "{}", error);
      },
      (e, _) => {
          if e.is_empty() && !bucket.is_empty() {
            // delete bucket
            // If the bucket has objects it will return an error
            let result = delete_bucket(bucket, client);
          } else {
              let error = format!("incorrect or missing request {}", e);
              println_color_quiet!(client.is_quiet, term::color::RED, "{}", error);
          }
      },
    }

    Ok(())
}

fn delete_bucket<P, D>(bucket: &str,
                       client: &Client<P, D>)
                       -> Result<(), S3Error>
                       where P: AwsCredentialsProvider,
                             D: DispatchSignedRequest {
    let request = DeleteBucketRequest { bucket: bucket.to_string() };

    match client.s3client.delete_bucket(&request) {
        Ok(_) => {
          println_color_quiet!(client.is_quiet, client.output.color, "Success");
          Ok(())
        },
        Err(e) => {
          println_color_quiet!(client.is_quiet, client.error.color, "{:#?}", e);
          Err(e)
        },
    }
}
