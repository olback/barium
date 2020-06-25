use {
    crate::{extract_cert_field_value, error::BariumResult},
    gtk_resources::UIResource,
    gtk::{ApplicationWindow, Dialog, Label, Stack, ResponseType, prelude::*},
    openssl::{x509::X509, nid::Nid, hash::MessageDigest}
};

#[derive(Debug, UIResource)]
#[resource="/net/olback/barium/ui/cert-info-dialog"]
pub struct CertificateDialog {
    pub cert_info_dialog: Dialog,
    pub stack: Stack,
    pub error_label: Label,
    pub issued_to_cn: Label,
    pub issued_to_o: Label,
    pub issued_to_ou: Label,
    pub issued_by_cn: Label,
    pub issued_by_o: Label,
    pub issued_by_ou: Label,
    pub validity_issued_on: Label,
    pub validity_expires_on: Label,
    pub fingerprint_sha_256: Label,
    pub fingerprint_sha_1: Label
}

impl CertificateDialog {

    pub fn build(main_window: ApplicationWindow) -> BariumResult<Self> {

        let inner = Self::load()?;
        inner.cert_info_dialog.set_transient_for(Some(&main_window));
        inner.cert_info_dialog.add_button("Ok", ResponseType::Ok);

        Ok(inner)

    }

    pub fn show(&self, der_bytes: &[u8]) {

        match self.from_der(der_bytes) {
            Ok(_) => self.stack.set_visible_child_name("cert"),
            Err(e) => {
                self.error_label.set_text(&format!("{}", e));
                self.stack.set_visible_child_name("error");
            }
        }

        match self.cert_info_dialog.run() {
            _ => self.cert_info_dialog.hide()
        }

    }

    fn from_der(&self, der_bytes: &[u8]) -> BariumResult<()> {

        let x509 = X509::from_der(der_bytes)?;

        let issued_to_cn = extract_cert_field_value!(x509.subject_name(), Nid::COMMONNAME);
        self.cert_info_dialog.set_title(&format!("Certificate {}", issued_to_cn));
        self.issued_to_cn.set_text(&issued_to_cn);

        let issued_to_o = extract_cert_field_value!(x509.subject_name(), Nid::ORGANIZATIONNAME);
        self.issued_to_o.set_text(&issued_to_o);

        let issued_to_ou = extract_cert_field_value!(x509.subject_name(), Nid::ORGANIZATIONALUNITNAME);
        self.issued_to_ou.set_text(&issued_to_ou);

        let issued_by_cn = extract_cert_field_value!(x509.issuer_name(), Nid::COMMONNAME);
        self.issued_by_cn.set_text(&issued_by_cn);

        let issued_by_o = extract_cert_field_value!(x509.issuer_name(), Nid::ORGANIZATIONNAME);
        self.issued_by_o.set_text(&issued_by_o);

        let issued_by_ou = extract_cert_field_value!(x509.issuer_name(), Nid::ORGANIZATIONALUNITNAME);
        self.issued_by_ou.set_text(&issued_by_ou);

        let issued_on = x509.not_before();
        self.validity_issued_on.set_text(&issued_on.to_string());

        let expires_on = x509.not_after();
        self.validity_expires_on.set_text(&expires_on.to_string());

        let fingerprint_sha_256 = x509.digest(MessageDigest::sha256())
            .map(|v| v.iter().map(|ref b| format!("{:02X}", b)).collect::<Vec<String>>().join(" "))
            .unwrap_or("<unavailable>".into());
        self.fingerprint_sha_256.set_text(&fingerprint_sha_256);

        let fingerprint_sha_1 = x509.digest(MessageDigest::sha1())
            .map(|v| v.iter().map(|ref b| format!("{:02X}", b)).collect::<Vec<String>>().join(" "))
            .unwrap_or("<unavailable>".into());
        self.fingerprint_sha_1.set_text(&fingerprint_sha_1);

        Ok(())

    }

}
