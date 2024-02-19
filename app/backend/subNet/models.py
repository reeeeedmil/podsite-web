from django.db import models
from django.core.validators import int_list_validator
# Create your models here.
from django.db.models import signals
from django.dispatch import receiver
from django.utils.text import slugify


class Net(models.Model):
    title = models.CharField(
        max_length=100, blank=True, null=True, unique=True)
    title_slug = models.SlugField(
        max_length=150, blank=True, null=True)
    parent = models.ForeignKey('Net',
                               on_delete=models.CASCADE,
                               blank=True,
                               null=True,
                               )

    network_address = models.CharField(max_length=15,
                                       validators=[
                                           int_list_validator(sep='.')
                                       ]
                                       )
    broadcast = models.CharField(max_length=15,
                                 validators=[
                                     int_list_validator(sep='.')
                                 ]
                                 )
    prefix = models.PositiveIntegerField()

    mask = models.CharField(max_length=15,
                            validators=[
                                int_list_validator(sep='.')
                            ]
                            )
    wildcard = models.CharField(max_length=15,
                                validators=[
                                    int_list_validator(sep='.')
                                ]
                                )

    def __str__(self):
        return f"id: {self.id}//net address: {self.network_address}\ntitle: {self.title}"


@receiver(signals.pre_save, sender=Net)
def populate_slug(sender, instance, **kwargs):
    instance.title_slug = slugify(instance.title)
